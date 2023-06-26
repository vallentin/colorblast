#![forbid(unsafe_code)]
#![forbid(elided_lifetimes_in_paths)]

mod lexers;

pub use text_scanner as scanner;

pub use self::lexers::*;

use std::fmt;
use std::ops::Range;

use text_scanner::Scanner;

#[derive(Eq, Clone)]
pub struct TokenSpan<'text> {
    text: &'text str,
    range: Range<usize>,
}

impl<'text> TokenSpan<'text> {
    #[inline]
    pub fn new(text: &'text str, range: Range<usize>) -> Self {
        Self { text, range }
    }

    #[inline]
    pub fn as_str(&self) -> &'text str {
        &self.text[self.range.clone()]
    }

    #[inline]
    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }

    #[inline]
    pub fn start(&self) -> usize {
        self.range.start
    }

    #[inline]
    pub fn end(&self) -> usize {
        self.range.end
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.range.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.range.is_empty()
    }
}

impl fmt::Debug for TokenSpan<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TokenSpan")
            .field("start", &self.range.start)
            .field("end", &self.range.end)
            .field("string", &self.as_str())
            .finish()
    }
}

impl fmt::Display for TokenSpan<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl PartialEq for TokenSpan<'_> {
    fn eq(&self, other: &Self) -> bool {
        (self.text.as_ptr() == other.text.as_ptr()) && (self.range == other.range)
    }
}

pub trait Lexer<'text> {
    type Token: ScanToken;

    #[inline]
    fn next_token(&mut self) -> Option<(Self::Token, TokenSpan<'text>)> {
        Self::Token::scan_token(self.scanner_mut())
    }

    #[inline]
    fn peek_token(&self) -> Option<(Self::Token, TokenSpan<'text>)> {
        self.scanner().peeking(Self::Token::scan_token)
    }

    #[inline]
    fn cursor_pos(&self) -> usize {
        self.scanner().cursor_pos()
    }

    #[inline]
    fn set_cursor_pos(&mut self, pos: usize) -> usize {
        self.scanner_mut().set_cursor_pos(pos)
    }

    #[inline]
    fn reset(&mut self) -> usize {
        self.set_cursor_pos(0)
    }

    fn scanner(&self) -> &Scanner<'text>;
    fn scanner_mut(&mut self) -> &mut Scanner<'text>;
}

pub trait ScanToken: Sized {
    fn scan_token<'text>(scanner: &mut Scanner<'text>) -> Option<(Self, TokenSpan<'text>)>;
}

macro_rules! impl_lexer_from_scanner {
    ($lifetime:lifetime, $lexer:ty, $token:ty, $scanner:ident) => {
        impl<$lifetime> $crate::Lexer<$lifetime> for $lexer {
            type Token = $token;

            #[inline]
            fn scanner(&self) -> &Scanner<$lifetime> {
                &self.$scanner
            }

            #[inline]
            fn scanner_mut(&mut self) -> &mut Scanner<$lifetime> {
                &mut self.$scanner
            }
        }

        $crate::impl_iter_for_lexer!($lifetime, $lexer);
    };
}

pub(crate) use impl_lexer_from_scanner;

macro_rules! impl_iter_for_lexer {
    ($lifetime:lifetime, $lexer:ty) => {
        impl<$lifetime> Iterator for $lexer {
            type Item = (
                <Self as $crate::Lexer<$lifetime>>::Token,
                TokenSpan<$lifetime>,
            );

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                $crate::Lexer::next_token(self)
            }
        }

        impl<$lifetime> std::iter::FusedIterator for $lexer {}
    };
}

pub(crate) use impl_iter_for_lexer;

pub(crate) trait ScannerExt<'text> {
    fn span(&self, range: Range<usize>) -> TokenSpan<'text>;
}

impl<'text> ScannerExt<'text> for Scanner<'text> {
    #[inline]
    fn span(&self, range: Range<usize>) -> TokenSpan<'text> {
        TokenSpan::new(self.text(), range)
    }
}
