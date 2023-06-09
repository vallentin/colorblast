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

pub(crate) trait ScannerExt<'text> {
    fn span(&self, range: Range<usize>) -> TokenSpan<'text>;
}

impl<'text> ScannerExt<'text> for Scanner<'text> {
    #[inline]
    fn span(&self, range: Range<usize>) -> TokenSpan<'text> {
        TokenSpan::new(self.text(), range)
    }
}
