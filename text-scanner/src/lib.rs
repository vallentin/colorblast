//! See [`Scanner`] docs for more information and its [methods]
//! for many examples.
//!
//! [methods]: Scanner#implementations

#![forbid(unsafe_code)]
#![forbid(elided_lifetimes_in_paths)]

#[cfg(feature = "ext")]
pub mod ext;

pub mod prelude {
    pub use super::{ScanResult, Scanner, ScannerItem, ScannerResult};
}

mod private {
    pub trait Sealed {}

    impl Sealed for crate::Scanner<'_> {}
    impl Sealed for str {}
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
///
/// **Note:** Cloning `Scanner` is essentially a copy, as it just contains
/// a `&str` and a `usize` for its `cursor`. However, `Copy` is not
/// implemented, to avoid accidentally copying immutable `Scanner`s.
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
        self.cursor < self.text.len()
    }

    /// Utility for turning a `Range<usize>` into `(Range<usize>, &'text str)`.
    /// Where `range` is the start end end byte index relative to [`text()`].
    ///
    /// The same as `(range.clone(), &self.text()[range])`.
    ///
    /// [`text()`]: Self::text
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
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> ScannerResult<'text, char> {
        let (r, c) = self.peek()?;
        self.cursor = r.end;
        Ok((r, c))
    }

    /// Returns the next [`char`] and its [`Range`], if any,
    /// without advancing the cursor position.
    ///
    /// See also [`peek_str()`], [`peek_nth()`], and [`peek_iter()`].
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
    ///
    /// [`peek_str()`]: Self::peek_str
    /// [`peek_nth()`]: Self::peek_nth
    /// [`peek_iter()`]: Self::peek_iter
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
    /// See also [`peek_str()`] and [`peek_iter()`].
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
    ///
    /// [`peek_str()`]: Self::peek_str
    /// [`peek_iter()`]: Self::peek_iter
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
    /// **Note:** This has the same lifetime as the original `text`,
    /// so the scanner can continue to be used while this exists.
    ///
    /// See also [`peek_str()`].
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
    ///
    /// [`peek_str()`]: Self::peek_str
    #[inline]
    pub fn peek_iter(&self) -> CharRangesOffset<'text> {
        self.remaining_text().char_ranges().offset(self.cursor)
    }

    /// Advances the scanner cursor and returns [`Ok`] with a string
    /// slice of the following `n` characters. If less than `n` are
    /// remaining, then [`Err`] is returned, with the [remaining text],
    /// if any, without advancing the cursor.
    ///
    /// **Note:** The returned string slice has the same lifetime as
    /// the original `text`, so the scanner can continue to be used
    /// while this exists.
    ///
    /// # Bytes vs Characters
    ///
    /// The [`Ok`] string slice contains `n` characters,
    /// i.e. where `n` matches <code>str.[chars()].[count()]</code>
    /// and **not** [`len()`] (which is the byte length of a string slice).
    ///
    /// Consider `"foo"` vs `"🦀🦀🦀"`, both string slices contain 3
    /// characters. However `"foo"` has a length of 3 bytes, while `"🦀🦀🦀"`
    /// has a length of 12 bytes, when encoded in UTF-8.
    ///
    /// # Panics
    ///
    /// Panics in non-optimized builds, if `n` is `0`.
    ///
    /// In optimized builds <code>Err(([cursor]..[cursor], &quot;&quot;))</code>
    /// is returned instead, regardless of whether there is any remaining
    /// characters.
    ///
    /// In short there is a <code>[debug_assert_ne!](n, 0)</code>.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// let mut scanner = Scanner::new("Foo Bar Baz");
    ///
    /// # assert_eq!(scanner.remaining_text(), "Foo Bar Baz");
    /// assert_eq!(scanner.next_str(3), Ok((0..3, "Foo")));
    /// assert_eq!(scanner.next_str(3), Ok((3..6, " Ba")));
    /// assert_eq!(scanner.next_str(3), Ok((6..9, "r B")));
    /// // Less than 3 characters are remaining, so `Err`
    /// // is returned
    /// assert_eq!(scanner.next_str(3), Err((9..11, "az")));
    /// # assert_eq!(scanner.remaining_text(), "az");
    /// # assert_eq!(scanner.next_str(2), Ok((9..11, "az")));
    /// # assert_eq!(scanner.remaining_text(), "");
    /// ```
    ///
    /// [remaining text]: Self::remaining_text
    /// [chars()]: str::chars
    /// [count()]: Iterator::count()
    /// [`len()`]: str::len
    /// [cursor]: Self::cursor_pos()
    #[inline]
    pub fn next_str(&mut self, chars: usize) -> ScannerResult<'text, &'text str> {
        let (r, s) = self.peek_str(chars)?;
        self.cursor = r.end;
        Ok((r, s))
    }

    /// Returns [`Ok`] with a string slice of the following `n` characters,
    /// if any, without advancing the cursor. If less than `n` are remaining,
    /// then [`Err`] is returned, with the [remaining text].
    ///
    /// **Note:** The returned string slice has the same lifetime as
    /// the original `text`, so the scanner can continue to be used
    /// while this exists.
    ///
    /// # Bytes vs Characters
    ///
    /// The [`Ok`] string slice contains `n` characters,
    /// i.e. where `n` matches <code>str.[chars()].[count()]</code>
    /// and **not** [`len()`] (which is the byte length of a string slice).
    ///
    /// Consider `"foo"` vs `"🦀🦀🦀"`, both string slices contain 3
    /// characters. However `"foo"` has a length of 3 bytes, while `"🦀🦀🦀"`
    /// has a length of 12 bytes, when encoded in UTF-8.
    ///
    /// # Panics
    ///
    /// Panics in non-optimized builds, if `n` is `0`.
    ///
    /// In optimized builds <code>Err(([cursor]..[cursor], &quot;&quot;))</code>
    /// is returned instead, regardless of whether there is any remaining
    /// characters.
    ///
    /// In short there is a <code>[debug_assert_ne!](n, 0)</code>.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// let mut scanner = Scanner::new("Hello 👋 World 🌏");
    ///
    /// assert_eq!(scanner.remaining_text(), "Hello 👋 World 🌏");
    /// // The emoji is a multi-byte character, thereby the returned
    /// // range has a length of 10 and not 7.
    /// assert_eq!(scanner.peek_str(7), Ok((0..10, "Hello 👋")));
    /// # assert_eq!(scanner.remaining_text(), "Hello 👋 World 🌏");
    ///
    /// assert_eq!(scanner.next(), Ok((0..1, 'H')));
    /// assert_eq!(scanner.next(), Ok((1..2, 'e')));
    ///
    /// assert_eq!(scanner.remaining_text(), "llo 👋 World 🌏");
    /// assert_eq!(scanner.peek_str(7), Ok((2..12, "llo 👋 W")));
    /// # assert_eq!(scanner.remaining_text(), "llo 👋 World 🌏");
    /// ```
    ///
    /// [remaining text]: Self::remaining_text
    /// [chars()]: str::chars
    /// [count()]: Iterator::count()
    /// [`len()`]: str::len
    /// [cursor]: Self::cursor_pos()
    #[inline]
    pub fn peek_str(&self, n: usize) -> ScannerResult<'text, &'text str> {
        debug_assert_ne!(n, 0, "`n` must be greater than 0");
        if n == 0 {
            return Err((self.cursor..self.cursor, ""));
        }
        let (last, _) = self.peek_nth(n - 1)?;
        let r = self.cursor..last.end;
        Ok(self.ranged_text(r))
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
    /// is returned instead, regardless of whether there is any remaining
    /// characters.
    ///
    /// In short there is a <code>[debug_assert!]\(!expected.is_empty())</code>.
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

    /// Advances the scanner cursor and returns `Ok` with the `&'text str`
    /// and its [`Range`], if the next characters matches the characters
    /// in `expected`. If not, then an `Err` is returned, with the longest
    /// matching substring and its [`Range`].
    ///
    /// **Note:** The returned string slice has the same lifetime as
    /// the original `text`, so the scanner can continue to be used
    /// while this exists.
    ///
    /// If `expected` is only 1 character, then use [`accept_char()`]
    /// instead.
    ///
    /// # Panics
    ///
    /// Panics in non-optimized builds, if `expected` is [empty].
    ///
    /// In optimized builds <code>Err(([cursor]..[cursor], &quot;&quot;))</code>
    /// is returned instead, regardless of whether there is any remaining
    /// characters.
    ///
    /// In short there is a <code>[debug_assert!]\(!expected.is_empty())</code>.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// let mut scanner = Scanner::new("FooBaaar");
    ///
    /// // The next 3 characters matches "Foo", so `Ok` is returned
    /// assert_eq!(scanner.accept_str("Foo"), Ok((0..3, "Foo")));
    ///
    /// // The next 3 characters is "Baa" not "Bar", so `Err` is
    /// // returned, with the longest matching part, i.e. "Ba"
    /// assert_eq!(scanner.accept_str("Bar"), Err((3..5, "Ba")));
    ///
    /// assert_eq!(scanner.remaining_text(), "Baaar");
    /// ```
    ///
    /// [`accept_char()`]: Self::accept_char
    /// [cursor]: Self::cursor_pos
    /// [empty]: https://doc.rust-lang.org/std/primitive.str.html#method.is_empty
    pub fn accept_str(&mut self, expected: &str) -> ScannerResult<'text, &'text str> {
        debug_assert!(!expected.is_empty(), "`expected` is empty");
        if expected.is_empty() {
            return Err((self.cursor..self.cursor, ""));
        }

        let start = self.cursor;

        let mut chars = self.peek_iter();
        for expected in expected.chars() {
            match chars.next() {
                Some((r, c)) if c == expected => {
                    self.cursor = r.end;
                }
                _ => {
                    let end = self.cursor;
                    self.cursor = start;
                    return Err(self.ranged_text(start..end));
                }
            }
        }

        Ok(self.ranged_text(start..self.cursor))
    }

    /// Advances the scanner cursor and returns `Ok` with the `&'text str`
    /// and its [`Range`], if the next characters matches any `&str`
    /// in `expected`. If not, then an `Err` is returned, with the longest
    /// matching substring and its [`Range`].
    ///
    /// **Warning:** The strings are tested in sequential order, thereby
    /// if `accept_str_any()` is called with e.g. `["foo", "foobar"]`,
    /// then `"foobar"` would never be tested, as `"foo"` would be
    /// matched and return `Ok` beforehand. Instead simply change the
    /// order of the strings into longest-to-shortest order,
    /// i.e. `["foo", "foobar"]` into `["foobar", "foo"]`.
    ///
    /// **Note:** The returned string slice has the same lifetime as
    /// the original `text`, so the scanner can continue to be used
    /// while this exists.
    ///
    /// If `expected` only contains 1 character strings, then use
    /// [`accept_char_any()`] instead.
    ///
    /// # Panics
    ///
    /// Panics in non-optimized builds, if `expected` is [empty],
    /// or if `expected` contains an [empty][empty2] `&str`.
    ///
    /// In optimized builds <code>Err(([cursor]..[cursor], &quot;&quot;))</code>
    /// is returned instead, regardless of whether there is any remaining
    /// characters.
    ///
    /// In short there is a <code>[debug_assert!]\(!expected.is_empty())</code>
    /// (along with a similar assertion for the strings).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// let mut scanner = Scanner::new("FooBarFooBaaar");
    ///
    /// let any = &["Foo", "Bar"];
    ///
    /// // The next 3 characters matches "Foo", so `Ok` is returned
    /// assert_eq!(scanner.accept_str_any(any), Ok((0..3, "Foo")));
    /// assert_eq!(scanner.accept_str_any(any), Ok((3..6, "Bar")));
    /// assert_eq!(scanner.accept_str_any(any), Ok((6..9, "Foo")));
    ///
    /// // The next 3 characters is "Baa" not "Foo" nor "Bar", so `Err`
    /// // is returned, with the longest matching part, i.e. "Ba"
    /// assert_eq!(scanner.accept_str_any(any), Err((9..11, "Ba")));
    ///
    /// assert_eq!(scanner.remaining_text(), "Baaar");
    /// ```
    ///
    /// [`accept_char_any()`]: Self::accept_char_any
    /// [cursor]: Self::cursor_pos
    /// [empty]: https://doc.rust-lang.org/std/primitive.slice.html#method.is_empty
    /// [empty2]: https://doc.rust-lang.org/std/primitive.str.html#method.is_empty
    pub fn accept_str_any(&mut self, expected: &[&str]) -> ScannerResult<'text, &'text str> {
        debug_assert!(!expected.is_empty(), "`expected` is empty");
        if expected.is_empty() {
            return Err((self.cursor..self.cursor, ""));
        }

        let mut max_end = self.cursor;
        for expected in expected {
            match self.accept_str(expected) {
                Ok((r, s)) => return Ok((r, s)),
                Err((r, _s)) => {
                    max_end = max_end.max(r.end);
                }
            }
        }

        let r = self.cursor..max_end;
        Err(self.ranged_text(r))
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

    /// Skips zero-to-many characters, while the next characters
    /// matches the characters in `expected` completely.
    ///
    /// **Note:** The returned string slice has the same lifetime as
    /// the original `text`, so the scanner can continue to be used
    /// while this exists.
    ///
    /// If `expected` is only 1 character, then use [`skip_while_char()`]
    /// instead.
    ///
    /// # Panics
    ///
    /// Panics in non-optimized builds, if `expected` is [empty].
    ///
    /// In optimized builds 0 characters are skipped, and
    /// <code>([cursor]..[cursor], &quot;&quot;)</code> is returned instead,
    /// regardless of whether there is any remaining characters.
    ///
    /// In short there is a <code>[debug_assert!]\(!expected.is_empty())</code>.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// let mut scanner = Scanner::new("FooFooFooBarBaz");
    /// assert_eq!(scanner.skip_while_str("Foo"), (0..9, "FooFooFoo"));
    /// assert_eq!(scanner.remaining_text(), "BarBaz");
    /// ```
    ///
    /// [`skip_while_char()`]: Self::skip_while_char
    /// [cursor]: Self::cursor_pos
    /// [empty]: https://doc.rust-lang.org/std/primitive.str.html#method.is_empty
    #[inline]
    pub fn skip_while_str(&mut self, expected: &str) -> ScannerItem<&'text str> {
        let start = self.cursor;

        while self.accept_str(expected).is_ok() {}

        self.ranged_text(start..self.cursor)
    }

    /// Skips zero-to-many characters, while the next characters
    /// matches the characters of any `&str` in `expected` completely.
    ///
    /// **Warning:** The strings are tested in sequential order, thereby
    /// if `skip_while_str_any()` is called with e.g. `["foo", "foobar"]`,
    /// then `"foobar"` would never be tested, as `"foo"` would be
    /// matched and continue beforehand. Instead simply change the
    /// order of the strings into longest-to-shortest order,
    /// i.e. `["foo", "foobar"]` into `["foobar", "foo"]`.
    ///
    /// **Note:** The returned string slice has the same lifetime as
    /// the original `text`, so the scanner can continue to be used
    /// while this exists.
    ///
    /// If `expected` only contains 1 character strings, then use
    /// [`skip_while_char_any()`] instead.
    ///
    /// # Panics
    ///
    /// Panics in non-optimized builds, if `expected` is [empty],
    /// or if `expected` contains an [empty][empty2] `&str`.
    ///
    /// In optimized builds 0 characters are skipped, and
    /// <code>([cursor]..[cursor], &quot;&quot;)</code> is returned instead,
    /// regardless of whether there is any remaining characters.
    ///
    /// In short there is a <code>[debug_assert!]\(!expected.is_empty())</code>
    /// (along with a similar assertion for the strings).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// let mut scanner = Scanner::new("FooBarFooBarFooBaaarBaz");
    /// assert_eq!(scanner.skip_while_str_any(&["Foo", "Bar"]), (0..15, "FooBarFooBarFoo"));
    /// assert_eq!(scanner.remaining_text(), "BaaarBaz");
    /// ```
    ///
    /// [`skip_while_char_any()`]: Self::skip_while_char_any
    /// [cursor]: Self::cursor_pos
    /// [empty]: https://doc.rust-lang.org/std/primitive.slice.html#method.is_empty
    /// [empty2]: https://doc.rust-lang.org/std/primitive.str.html#method.is_empty
    #[inline]
    pub fn skip_while_str_any(&mut self, expected: &[&str]) -> ScannerItem<&'text str> {
        let start = self.cursor;

        while self.accept_str_any(expected).is_ok() {}

        self.ranged_text(start..self.cursor)
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

    /// Skips zero-to-many characters, until the next characters
    /// matches the characters in `expected` completely.
    ///
    /// **Note:** The returned string slice has the same lifetime as
    /// the original `text`, so the scanner can continue to be used
    /// while this exists.
    ///
    /// If `expected` is only 1 character, then use [`skip_until_char()`]
    /// instead.
    ///
    /// # Panics
    ///
    /// Panics in non-optimized builds, if `expected` is [empty].
    ///
    /// In optimized builds 0 characters are skipped, and
    /// <code>([cursor]..[cursor], &quot;&quot;)</code> is returned instead,
    /// regardless of whether there is any remaining characters.
    ///
    /// In short there is a <code>[debug_assert!]\(!expected.is_empty())</code>.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// let mut scanner = Scanner::new("FooFooFooBarBaz");
    /// assert_eq!(scanner.skip_until_str("Bar"), (0..9, "FooFooFoo"));
    /// assert_eq!(scanner.remaining_text(), "BarBaz");
    /// ```
    ///
    /// [`skip_until_char()`]: Self::skip_until_char
    /// [cursor]: Self::cursor_pos
    /// [empty]: https://doc.rust-lang.org/std/primitive.str.html#method.is_empty
    pub fn skip_until_str(&mut self, expected: &str) -> ScannerItem<&'text str> {
        let remaining_text = self.remaining_text();
        let end = remaining_text
            .find(expected)
            .unwrap_or(remaining_text.len());

        let start = self.cursor;
        self.cursor = end;

        self.ranged_text(start..end)
    }

    /// Skips zero-to-many characters, until the next characters
    /// matches the characters of any `&str` in `expected` completely.
    ///
    /// **Warning:** The strings are tested in sequential order, thereby
    /// if `skip_until_str_any()` is called with e.g. `["foo", "foobar"]`,
    /// then `"foobar"` would never be tested, as `"foo"` would be
    /// matched and continue beforehand. Instead simply change the
    /// order of the strings into longest-to-shortest order,
    /// i.e. `["foo", "foobar"]` into `["foobar", "foo"]`.
    ///
    /// **Note:** The returned string slice has the same lifetime as
    /// the original `text`, so the scanner can continue to be used
    /// while this exists.
    ///
    /// If `expected` only contains 1 character strings, then use
    /// [`skip_until_char_any()`] instead.
    ///
    /// # Panics
    ///
    /// Panics in non-optimized builds, if `expected` is [empty],
    /// or if `expected` contains an [empty][empty2] `&str`.
    ///
    /// In optimized builds 0 characters are skipped, and
    /// <code>([cursor]..[cursor], &quot;&quot;)</code> is returned instead,
    /// regardless of whether there is any remaining characters.
    ///
    /// In short there is a <code>[debug_assert!]\(!expected.is_empty())</code>
    /// (along with a similar assertion for the strings).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use text_scanner::Scanner;
    /// let mut scanner = Scanner::new("FooBarFooBarFooBaaarBaz");
    /// assert_eq!(scanner.skip_until_str_any(&["Baaar", "Baz"]), (0..15, "FooBarFooBarFoo"));
    /// assert_eq!(scanner.remaining_text(), "BaaarBaz");
    /// ```
    ///
    /// [`skip_until_char_any()`]: Self::skip_until_char_any
    /// [cursor]: Self::cursor_pos
    /// [empty]: https://doc.rust-lang.org/std/primitive.slice.html#method.is_empty
    /// [empty2]: https://doc.rust-lang.org/std/primitive.str.html#method.is_empty
    pub fn skip_until_str_any(&mut self, expected: &[&str]) -> ScannerItem<&'text str> {
        let start = self.cursor;

        while self.has_remaining_text() {
            if let Ok((r, _)) = self.accept_str_any(expected) {
                self.cursor = r.start;
                break;
            }

            _ = self.next();
        }

        self.ranged_text(start..self.cursor)
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

    /// Calls `f` with a <code>&mut [Scanner]</code> of this
    /// <code>&[Scanner]</code>, i.e. a [`Scanner`] with the
    /// same [`text()`], [`remaining_text()`], and [`cursor_pos()`].
    ///
    /// [`text()`]: Self::text
    /// [`remaining_text()`]: Self::remaining_text
    /// [`cursor_pos()`]: Self::cursor_pos
    pub fn peeking<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        let mut scanner = self.clone();
        f(&mut scanner)
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

#[allow(clippy::wrong_self_convention)]
pub(crate) trait CharExt {
    // `std::char::is_ascii_octdigit` is unstable
    fn is_ascii_octdigit(self) -> bool;

    fn is_ascii_bindigit(self) -> bool;
}

impl CharExt for char {
    #[inline]
    fn is_ascii_octdigit(self) -> bool {
        matches!(self, '0'..='7')
    }

    #[inline]
    fn is_ascii_bindigit(self) -> bool {
        matches!(self, '0' | '1')
    }
}

// If you are looking for tests, then the majority
// are implemented in the form of doc tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accept_str_any_order() {
        let mut scanner = Scanner::new("FooBarBaz");

        #[rustfmt::skip]
        assert_eq!(scanner.accept_str_any(&["Foo", "FooBar"]), Ok((0..3, "Foo")));
        assert_eq!(scanner.remaining_text(), "BarBaz");

        scanner.reset();

        #[rustfmt::skip]
        assert_eq!(scanner.accept_str_any(&["FooBar", "Foo"]), Ok((0..6, "FooBar")));
        assert_eq!(scanner.remaining_text(), "Baz");
    }
}
