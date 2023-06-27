//! Implementation of various lexers, for generating [`Token`]s by classifying
//! [`TokenSpan`]s of some input.
//!
//! # Warning
//!
//! The lexers implemented in [`colorblast::lexers`] are intended to be used for
//! syntax highlighting, where inspecting [`TokenSpan`]s and classifying them
//! more uniquely into a more generic [`Token`] variant is desired.
//!
//! [`colorblast::lexers`]: self

pub mod prelude {
    pub use super::Lexer;
}

mod json;
mod jsonc;
mod rust;

pub use self::json::*;
pub use self::jsonc::*;
pub use self::rust::*;

use crate::{Token, TokenSpan};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[non_exhaustive]
pub enum Lexer {
    /// If the JSON might contain JavaScript-like comments, then
    /// use [`Lexer::JsonC`] instead, i.e. [JSON with Comments].
    ///
    /// [JSON with Comments]: https://code.visualstudio.com/docs/languages/json#_json-with-comments
    Json,
    /// [JSON with Comments].
    ///
    /// [JSON with Comments]: https://code.visualstudio.com/docs/languages/json#_json-with-comments
    JsonC,
    Rust,
}

impl Lexer {
    pub(crate) fn into_lexer<'text>(
        self,
        text: &'text str,
    ) -> Box<dyn Iterator<Item = (Token, TokenSpan<'text>)> + 'text> {
        match self {
            Self::Json => Box::new(JsonLexer::new(text)),
            Self::JsonC => Box::new(JsonCLexer::new(text)),
            Self::Rust => Box::new(RustLexer::new(text)),
        }
    }
}

macro_rules! impl_iter {
    ($lifetime:lifetime, $ty:ty) => {
        impl<$lifetime> Iterator for $ty {
            type Item = (Token, TokenSpan<$lifetime>);

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                self.next_token()
            }
        }

        impl<$lifetime> std::iter::FusedIterator for $ty {}
    };
}

pub(crate) use impl_iter;

trait LexerExt<'text, Tok>
where
    Self: Clone,
    Self: Iterator<Item = (Tok, TokenSpan<'text>)>,
    Tok: Copy,
{
    #[inline]
    fn peek_find<P>(&self, mut predicate: P) -> Option<Self::Item>
    where
        P: FnMut((Tok, &TokenSpan<'text>)) -> bool,
    {
        self.clone().find(|(tok, span)| predicate((*tok, span)))
    }

    #[inline]
    fn peek_find_token<P>(&self, mut predicate: P) -> Option<Self::Item>
    where
        P: FnMut(Tok) -> bool,
    {
        self.clone().find(move |(tok, _span)| predicate(*tok))
    }
}

impl<'text, Tok, I> LexerExt<'text, Tok> for I
where
    I: Clone,
    I: Iterator<Item = (Tok, TokenSpan<'text>)>,
    Tok: Copy,
{
}
