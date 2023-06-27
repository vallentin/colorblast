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

mod rust;

pub use self::rust::*;

use crate::{Token, TokenSpan};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[non_exhaustive]
pub enum Lexer {
    Rust,
}

impl Lexer {
    pub(crate) fn into_lexer<'text>(
        self,
        text: &'text str,
    ) -> Box<dyn Iterator<Item = (Token, TokenSpan<'text>)> + 'text> {
        match self {
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
