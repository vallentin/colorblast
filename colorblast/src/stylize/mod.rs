// mod rust;

// pub use self::rust::*;

use any_lexer::TokenSpan;

use crate::style::Style;

pub trait StylizeToken {
    fn style(&self, span: &TokenSpan<'_>) -> Style;
}
