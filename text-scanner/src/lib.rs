//! See [`Scanner`] docs for more information and its [methods]
//! for many examples.
//!
//! [methods]: Scanner#implementations

#![forbid(unsafe_code)]
#![forbid(elided_lifetimes_in_paths)]

pub mod ext;

pub mod prelude {
    pub use super::{ScanResult, Scanner, ScannerItem, ScannerResult};
}

pub use char_ranges::{CharRanges, CharRangesExt, CharRangesOffset};

use std::ops::Range;

pub type ScannerItem<T> = (Range<usize>, T);

pub type ScannerResult<'text, T> = Result<ScannerItem<T>, ScannerItem<&'text str>>;

pub type ScanResult<'text> = Result<(), ScannerItem<&'text str>>;

/// A `Scanner` is a UTF-8 [`char`] text scanner, implementing various methods
/// for scanning a string slice, as well as backtracking capabilities, which
/// can be used to implement lexers for tokenizing text or code. It is essentially
/// just a fancy wrapper around [`CharRanges`].
#[derive(Clone, Debug)]
pub struct Scanner<'text> {
    text: &'text str,
    cursor: usize,
}

impl<'text> Scanner<'text> {
    /// Constructs a new [`Scanner`] with `text`.
    #[inline]
    pub fn new(text: &'text str) -> Self {
        Self { text, cursor: 0 }
    }

    /// Returns the `text` the scanner was constructed with.
    ///
    /// **Note:** This has the same lifetime as the original `text`,
    /// so the scanner can continue to be used while this exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// let mut scanner = Scanner::new("Hello World");
    ///
    /// assert_eq!(scanner.next(), Ok((0..1, 'H')));
    /// assert_eq!(scanner.next(), Ok((1..2, 'e')));
    ///
    /// assert_eq!(scanner.text(), "Hello World");
    /// assert_eq!(scanner.remaining_text(), "llo World");
    /// ```
    #[inline]
    pub fn text(&self) -> &'text str {
        self.text
    }

    /// Returns the remaining `text` of the scanner, i.e. the [`text()`]
    /// after [`cursor_pos()`], in other words
    /// <code style="white-space: nowrap;">self.[text()]\[self.[cursor_pos()]..]</code>.
    ///
    /// **Note:** This has the same lifetime as the original `text`,
    /// so the scanner can continue to be used while this exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// let mut scanner = Scanner::new("Hello World");
    ///
    /// assert_eq!(scanner.text(), "Hello World");
    /// assert_eq!(scanner.remaining_text(), "Hello World");
    ///
    /// assert_eq!(scanner.next(), Ok((0..1, 'H')));
    /// assert_eq!(scanner.next(), Ok((1..2, 'e')));
    ///
    /// assert_eq!(scanner.text(), "Hello World");
    /// assert_eq!(scanner.remaining_text(), "llo World");
    /// ```
    ///
    /// [`text()`]: Self::text
    /// [text()]: Self::text
    /// [`cursor_pos()`]: Self::cursor_pos
    /// [cursor_pos()]: Self::cursor_pos
    #[inline]
    pub fn remaining_text(&self) -> &'text str {
        &self.text[self.cursor..]
    }

    /// Returns `true` if [`remaining_text()`] has text, i.e.
    /// if it is not [empty].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// let mut scanner = Scanner::new("Foo");
    ///
    /// # assert_eq!(scanner.text(), "Foo");
    /// assert_eq!(scanner.remaining_text(), "Foo");
    /// assert_eq!(scanner.has_remaining_text(), true);
    ///
    /// assert_eq!(scanner.next(), Ok((0..1, 'F')));
    /// assert_eq!(scanner.next(), Ok((1..2, 'o')));
    /// assert_eq!(scanner.next(), Ok((2..3, 'o')));
    ///
    /// # assert_eq!(scanner.text(), "Foo");
    /// assert_eq!(scanner.remaining_text(), "");
    /// assert_eq!(scanner.has_remaining_text(), false);
    /// ```
    ///
    /// [`remaining_text()`]: Self::remaining_text
    /// [empty]: https://doc.rust-lang.org/std/primitive.str.html#method.is_empty
    #[inline]
    pub fn has_remaining_text(&self) -> bool {
        !self.text[self.cursor..].is_empty()
    }

    #[inline]
    pub fn ranged_text(&self, range: Range<usize>) -> ScannerItem<&'text str> {
        (range.clone(), &self.text[range])
    }

    /// Returns the current cursor position of the
    /// scanner, i.e. the byte offset into [`text()`].
    ///
    /// [`text()`]: Self::text
    #[inline]
    pub fn cursor_pos(&self) -> usize {
        self.cursor
    }

    /// Replaces the current cursor position with `pos`,
    /// while returning the old cursor position.
    ///
    /// # Panics
    ///
    /// If `pos` is not at a valid UTF-8 sequence boundary,
    /// then the next operation using the cursor position
    /// will panic.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// let mut scanner = Scanner::new("Hello World");
    ///
    /// assert_eq!(scanner.next(), Ok((0..1, 'H')));
    ///
    /// let backtrack = scanner.cursor_pos();
    ///
    /// assert_eq!(scanner.next(), Ok((1..2, 'e')));
    /// assert_eq!(scanner.next(), Ok((2..3, 'l')));
    /// assert_eq!(scanner.next(), Ok((3..4, 'l')));
    ///
    /// scanner.set_cursor_pos(backtrack);
    ///
    /// assert_eq!(scanner.next(), Ok((1..2, 'e')));
    /// assert_eq!(scanner.next(), Ok((2..3, 'l')));
    /// assert_eq!(scanner.next(), Ok((3..4, 'l')));
    /// ```
    #[inline]
    pub fn set_cursor_pos(&mut self, pos: usize) -> usize {
        let old_pos = self.cursor;
        self.cursor = pos;
        old_pos
    }

    /// Resets the cursor position to the start, while returning
    /// the old cursor position.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// # let mut scanner = Scanner::new("Hello World");
    /// # assert_eq!(scanner.next(), Ok((0..1, 'H')));
    /// # assert_eq!(scanner.next(), Ok((1..2, 'e')));
    /// # assert_eq!(scanner.remaining_text(), "llo World");
    /// let old_pos = scanner.reset();
    /// // same as
    /// let old_pos = scanner.set_cursor_pos(0);
    /// # assert_eq!(scanner.remaining_text(), "Hello World");
    /// # assert_eq!(scanner.next(), Ok((0..1, 'H')));
    /// ```
    #[inline]
    pub fn reset(&mut self) -> usize {
        self.set_cursor_pos(0)
    }

    /// Advances the scanner cursor and returns the next
    /// [`char`] and its [`Range`], if any.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// let mut scanner = Scanner::new("Hello");
    ///
    /// assert_eq!(scanner.next(), Ok((0..1, 'H')));
    /// assert_eq!(scanner.next(), Ok((1..2, 'e')));
    ///
    /// assert_eq!(scanner.remaining_text(), "llo");
    ///
    /// assert_eq!(scanner.next(), Ok((2..3, 'l')));
    /// assert_eq!(scanner.next(), Ok((3..4, 'l')));
    /// assert_eq!(scanner.next(), Ok((4..5, 'o')));
    /// assert_eq!(scanner.next(), Err((5..5, "")));
    ///
    /// assert_eq!(scanner.remaining_text(), "");
    /// ```
    #[inline]
    pub fn next(&mut self) -> ScannerResult<'text, char> {
        let (r, c) = self.peek()?;
        self.cursor = r.end;
        Ok((r, c))
    }

    /// Returns the next [`char`] and its [`Range`], if any,
    /// without advancing the cursor position.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// let mut scanner = Scanner::new("Hello World");
    ///
    /// assert_eq!(scanner.peek(), Ok((0..1, 'H')));
    /// assert_eq!(scanner.peek(), Ok((0..1, 'H')));
    ///
    /// assert_eq!(scanner.next(), Ok((0..1, 'H')));
    ///
    /// assert_eq!(scanner.peek(), Ok((1..2, 'e')));
    /// assert_eq!(scanner.peek(), Ok((1..2, 'e')));
    ///
    /// assert_eq!(scanner.remaining_text(), "ello World");
    /// ```
    #[inline]
    pub fn peek(&self) -> ScannerResult<'text, char> {
        match self.peek_iter().next() {
            Some((r, c)) => Ok((r, c)),
            // No character remaining
            None => Err((self.cursor..self.cursor, "")),
        }
    }

    /// Returns the `n`th [`char`] and its [`Range`], if any,
    /// without advancing the cursor position.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// let mut scanner = Scanner::new("Hello World");
    ///
    /// assert_eq!(scanner.peek_nth(0), Ok((0..1, 'H')));
    /// assert_eq!(scanner.peek_nth(1), Ok((1..2, 'e')));
    /// assert_eq!(scanner.peek_nth(2), Ok((2..3, 'l')));
    ///
    /// assert_eq!(scanner.peek_nth(6), Ok((6..7, 'W')));
    ///
    /// assert_eq!(scanner.next(), Ok((0..1, 'H')));
    ///
    /// assert_eq!(scanner.remaining_text(), "ello World");
    /// ```
    #[inline]
    pub fn peek_nth(&self, n: usize) -> ScannerResult<'text, char> {
        match self.peek_iter().nth(n) {
            Some((r, c)) => Ok((r, c)),
            None => Err(self.ranged_text(self.cursor..self.text.len())),
        }
    }

    /// Returns an iterator that produces all the remaining [`char`]s
    /// and their [`Range`]s, if any, without advancing the cursor position.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// let mut scanner = Scanner::new("Hello World");
    ///
    /// assert_eq!(scanner.next(), Ok((0..1, 'H')));
    /// assert_eq!(scanner.remaining_text(), "ello World");
    ///
    /// let mut peek = scanner.peek_iter();
    /// assert_eq!(peek.next(), Some((1..2, 'e')));
    /// assert_eq!(peek.next(), Some((2..3, 'l')));
    /// assert_eq!(peek.next(), Some((3..4, 'l')));
    /// assert_eq!(scanner.remaining_text(), "ello World");
    ///
    /// assert_eq!(scanner.next(), Ok((1..2, 'e')));
    /// assert_eq!(scanner.next(), Ok((2..3, 'l')));
    /// assert_eq!(scanner.remaining_text(), "lo World");
    /// ```
    #[inline]
    pub fn peek_iter(&self) -> CharRangesOffset<'text> {
        self.remaining_text().char_ranges().offset(self.cursor)
    }

    /// Advances the scanner cursor and returns the next
    /// [`char`] and its [`Range`], if `f(c)` returns `true`
    /// where `c` is the next character.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// let mut scanner = Scanner::new("Hello World");
    ///
    /// assert_eq!(scanner.accept_if(char::is_alphabetic), Ok((0..1, 'H')));
    /// assert_eq!(scanner.accept_if(char::is_alphabetic), Ok((1..2, 'e')));
    /// assert_eq!(scanner.accept_if(char::is_alphabetic), Ok((2..3, 'l')));
    /// assert_eq!(scanner.accept_if(char::is_alphabetic), Ok((3..4, 'l')));
    /// assert_eq!(scanner.accept_if(char::is_alphabetic), Ok((4..5, 'o')));
    /// assert_eq!(scanner.accept_if(char::is_alphabetic), Err((5..5, "")));
    ///
    /// assert_eq!(scanner.remaining_text(), " World");
    /// ```
    #[inline]
    pub fn accept_if<F>(&mut self, f: F) -> ScannerResult<'text, char>
    where
        F: FnOnce(char) -> bool,
    {
        let (r, c) = self.peek()?;
        if f(c) {
            self.cursor = r.end;
            Ok((r, c))
        } else {
            Err((self.cursor..self.cursor, ""))
        }
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) fn accept_if_ext<A, Args>(&mut self, accept: A) -> ScannerResult<'text, char>
    where
        A: ScanOne<Args>,
    {
        self.accept_if(|c| accept.scan_one(c))
    }

    /// Advances the scanner cursor and returns the next
    /// [`char`] and its [`Range`], if the next character
    /// matches `expected`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// let mut scanner = Scanner::new("Hello World");
    ///
    /// assert_eq!(scanner.accept_char('H'), Ok((0..1, 'H')));
    /// assert_eq!(scanner.accept_char('E'), Err((1..1, "")));
    /// assert_eq!(scanner.accept_char('e'), Ok((1..2, 'e')));
    /// assert_eq!(scanner.accept_char('W'), Err((2..2, "")));
    ///
    /// assert_eq!(scanner.remaining_text(), "llo World");
    /// ```
    #[inline]
    pub fn accept_char(&mut self, expected: char) -> ScannerResult<'text, char> {
        self.accept_if(|c| c == expected)
    }

    /// Advances the scanner cursor and returns the next
    /// [`char`] and its [`Range`], if the next character
    /// matches any `char` produced by `expected`.
    ///
    /// # Panics
    ///
    /// Panics in non-optimized builds, if `expected` is [empty].
    ///
    /// In optimized builds <code>Err(([cursor]..[cursor], &quot;&quot;))</code>
    /// is returned instead.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// let mut scanner = Scanner::new("Hello World");
    ///
    /// let any = &['H', 'e', 'l', 'o', ' '];
    /// assert_eq!(scanner.accept_char_any(any), Ok((0..1, 'H')));
    /// assert_eq!(scanner.accept_char_any(any), Ok((1..2, 'e')));
    /// assert_eq!(scanner.accept_char_any(any), Ok((2..3, 'l')));
    /// assert_eq!(scanner.accept_char_any(any), Ok((3..4, 'l')));
    /// assert_eq!(scanner.accept_char_any(any), Ok((4..5, 'o')));
    /// assert_eq!(scanner.accept_char_any(any), Ok((5..6, ' ')));
    /// assert_eq!(scanner.accept_char_any(any), Err((6..6, "")));
    ///
    /// assert_eq!(scanner.remaining_text(), "World");
    /// ```
    ///
    /// [cursor]: Self::cursor_pos
    /// [empty]: https://doc.rust-lang.org/std/primitive.slice.html#method.is_empty
    pub fn accept_char_any(&mut self, expected: &[char]) -> ScannerResult<'text, char> {
        debug_assert!(!expected.is_empty(), "`expected` is empty");
        let (r, c) = self.peek()?;
        if expected.contains(&c) {
            self.cursor = r.end;
            Ok((r, c))
        } else {
            Err((self.cursor..self.cursor, ""))
        }
    }

    /// Advances the scanner cursor and skips zero-to-many characters,
    /// **while** `f(c)` returns `true`, where `c` is the [remaining characters]
    /// in sequential order.
    ///
    /// Returns the string slice and its [`Range`], of the matched
    /// (i.e. skipped) characters.
    ///
    /// Returns <code>([cursor]..[cursor], &quot;&quot;)</code> if 0 characters
    /// were matched (i.e. skipped).
    ///
    /// **Note:** The returned string slice has the same lifetime as
    /// the original `text`, so the scanner can continue to be used
    /// while this exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// let mut scanner = Scanner::new("Hello World");
    ///
    /// // Skip all alphabetic characters
    /// assert_eq!(scanner.skip_while(|c| c.is_alphabetic()), (0..5, "Hello"));
    ///
    /// // Returns an empty range and an empty string slice
    /// // since 0 characters were skipped
    /// assert_eq!(scanner.skip_while(|c| c.is_alphabetic()), (5..5, ""));
    ///
    /// // Skip 1 whitespace character
    /// assert_eq!(scanner.skip_while(char::is_whitespace), (5..6, " "));
    ///
    /// assert_eq!(scanner.remaining_text(), "World");
    /// ```
    ///
    /// [remaining characters]: Self::remaining_text
    /// [cursor]: Self::cursor_pos
    pub fn skip_while<F>(&mut self, mut f: F) -> ScannerItem<&'text str>
    where
        F: FnMut(char) -> bool,
    {
        let start = self.cursor;

        for (r, c) in self.peek_iter() {
            if f(c) {
                self.cursor = r.end;
            } else {
                break;
            }
        }

        let r = start..self.cursor;
        self.ranged_text(r)
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) fn skip_while_ext<A, Args>(&mut self, mut skip: A) -> ScannerItem<&'text str>
    where
        A: ScanMany<Args>,
    {
        self.skip_while(|c| skip.scan_many(c))
    }

    /// Skips zero-to-many characters matching `expected`, same as:
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// # let mut scanner = Scanner::new("Hello World");
    /// # let expected = 'H';
    /// scanner.skip_while(|c| c == expected);
    /// # assert_eq!(scanner.remaining_text(), "ello World");
    /// ```
    #[inline]
    pub fn skip_while_char(&mut self, expected: char) -> ScannerItem<&'text str> {
        self.skip_while(|c| c == expected)
    }

    /// Skips zero-to-many characters, which match any
    /// character in `expected`, same as:
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// # let mut scanner = Scanner::new("Hello World");
    /// # let expected = ['H', 'e', 'L'];
    /// scanner.skip_while(|c| expected.contains(&c));
    /// # assert_eq!(scanner.remaining_text(), "llo World");
    /// ```
    #[inline]
    pub fn skip_while_char_any(&mut self, expected: &[char]) -> ScannerItem<&'text str> {
        self.skip_while(|c| expected.contains(&c))
    }

    /// Advances the scanner cursor and skips zero-to-many characters,
    /// **while** `f(c)` returns `false`, where `c` is the [remaining characters]
    /// in sequential order.
    ///
    /// Returns the string slice and its [`Range`], of the matched
    /// (i.e. skipped) characters.
    ///
    /// Returns <code>([cursor]..[cursor], &quot;&quot;)</code> if 0 characters
    /// were matched (i.e. skipped).
    ///
    /// **Note:** The returned string slice has the same lifetime as
    /// the original `text`, so the scanner can continue to be used
    /// while this exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// let mut scanner = Scanner::new("Hello World");
    ///
    /// // Skip all characters until a whitespace is found
    /// assert_eq!(scanner.skip_until(|c| c.is_whitespace()), (0..5, "Hello"));
    ///
    /// // Returns an empty range and an empty string slice
    /// // since 0 characters were skipped
    /// assert_eq!(scanner.skip_until(|c| c.is_whitespace()), (5..5, ""));
    ///
    /// // Skip 1 whitespace character
    /// assert_eq!(scanner.skip_until(char::is_alphabetic), (5..6, " "));
    ///
    /// assert_eq!(scanner.remaining_text(), "World");
    /// ```
    ///
    /// [remaining characters]: Self::remaining_text
    /// [cursor]: Self::cursor_pos
    #[inline]
    pub fn skip_until<F>(&mut self, mut f: F) -> ScannerItem<&'text str>
    where
        F: FnMut(char) -> bool,
    {
        self.skip_while(|c| !f(c))
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) fn skip_until_ext<A, Args>(&mut self, mut skip: A) -> ScannerItem<&'text str>
    where
        A: ScanMany<Args>,
    {
        self.skip_until(|c| skip.scan_many(c))
    }

    /// Skips zero-to-many characters, until the next character
    /// matches `expected`, same as:
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// # let mut scanner = Scanner::new("Hello World");
    /// # let expected = ' ';
    /// scanner.skip_until(|c| c == expected);
    /// # assert_eq!(scanner.remaining_text(), " World");
    /// ```
    #[inline]
    pub fn skip_until_char(&mut self, expected: char) -> ScannerItem<&'text str> {
        self.skip_until(|c| c == expected)
    }

    /// Skips zero-to-many characters, until the next character
    /// match any in `expected`, same as:
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// # let mut scanner = Scanner::new("Hello World");
    /// # let expected = [' ', 'o'];
    /// scanner.skip_until(|c| expected.contains(&c));
    /// # assert_eq!(scanner.remaining_text(), "o World");
    /// ```
    #[inline]
    pub fn skip_until_char_any(&mut self, expected: &[char]) -> ScannerItem<&'text str> {
        self.skip_until(|c| expected.contains(&c))
    }

    /// Skips zero-to-many characters, while the next character
    /// is a [whitespace], same as:
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// # let mut scanner = Scanner::new("  Hello World");
    /// scanner.skip_while(char::is_whitespace);
    /// # assert_eq!(scanner.remaining_text(), "Hello World");
    /// ```
    ///
    /// [whitespace]: https://doc.rust-lang.org/std/primitive.char.html#method.is_whitespace
    #[inline]
    pub fn skip_whitespace(&mut self) -> ScannerItem<&'text str> {
        self.skip_while(char::is_whitespace)
    }

    /// Advances the cursor if `f()` returns `Ok`, otherwise on `Err` the
    /// cursor position is backtracked to before `f()` was called.
    ///
    /// Utility for scanning [tokens], where an unexpected character during
    /// scanning, should restore the cursor position before the the scan
    /// was started.
    ///
    /// Additionally, returns `Err` if `f()` returns `Ok`, without advancing
    /// the cursor position.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use text_scanner::{Scanner, ScannerItem};
    /// fn scan_word<'text>(scanner: &mut Scanner<'text>) -> Result<(), ScannerItem<&'text str>> {
    ///     // Get next char if alphabetic or return err
    ///     let (first, _c) = scanner.accept_if(char::is_alphabetic)?;
    ///     // Skip zero-to-many alphabetic characters
    ///     let (last, _s) = scanner.skip_while(char::is_alphabetic);
    ///     Ok(())
    /// }
    ///
    /// let text = "Hello World";
    /// let mut scanner = Scanner::new(text);
    ///
    /// assert_eq!(scanner.scan_with(scan_word), Ok((0..5, "Hello")));
    /// assert_eq!(scanner.scan_with(scan_word), Err((5..5, "")));
    /// assert_eq!(scanner.next(), Ok((5..6, ' ')));
    /// assert_eq!(scanner.scan_with(scan_word), Ok((6..11, "World")));
    /// # assert_eq!(scanner.remaining_text(), "");
    /// ```
    ///
    /// [tokens]: https://en.wikipedia.org/wiki/Lexical_analysis#Token
    #[inline]
    pub fn scan_with<F>(&mut self, f: F) -> ScannerResult<'text, &'text str>
    where
        F: FnOnce(&mut Self) -> ScanResult<'text>,
    {
        let start = self.cursor;

        let mut scanner = self.clone();

        match f(&mut scanner) {
            Ok(()) => {
                self.cursor = scanner.cursor;

                if self.cursor == start {
                    return Err((start..start, ""));
                }

                let r = start..self.cursor;
                Ok(self.ranged_text(r))
            }
            Err((last, _last_s)) => {
                let r = self.cursor..last.end;
                Err(self.ranged_text(r))
            }
        }
    }
}

// Currently not publicly exported, as using e.g. `accept_if()` with a
// closure would require specifying types more often than desired.
pub(crate) trait ScanOne<Args> {
    fn scan_one(self, next: char) -> bool;
}

impl<F> ScanOne<char> for F
where
    F: FnOnce(char) -> bool,
{
    #[inline]
    fn scan_one(self, next: char) -> bool {
        self(next)
    }
}

impl<F> ScanOne<&char> for F
where
    F: FnOnce(&char) -> bool,
{
    #[inline]
    fn scan_one(self, next: char) -> bool {
        self(&next)
    }
}

// Currently not publicly exported, as using e.g. `skip_while()` with a
// closure would require specifying types more often than desired.
pub(crate) trait ScanMany<Args>: ScanOne<Args> {
    fn scan_many(&mut self, next: char) -> bool;
}

impl<F> ScanMany<char> for F
where
    F: FnMut(char) -> bool,
{
    #[inline]
    fn scan_many(&mut self, next: char) -> bool {
        self(next)
    }
}

impl<F> ScanMany<&char> for F
where
    F: FnMut(&char) -> bool,
{
    #[inline]
    fn scan_many(&mut self, next: char) -> bool {
        self(&next)
    }
}

// If you are looking for tests, then they are
// all implemented in the form of doc tests
