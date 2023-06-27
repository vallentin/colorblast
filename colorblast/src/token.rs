pub mod prelude {
    pub use any_lexer::TokenSpan;

    pub use super::Token;
}

pub use any_lexer::TokenSpan;

use crate::style::Style;
use crate::stylize::StylizeToken;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[non_exhaustive]
pub enum Token {
    Space,
    Comment,
    Ident,
    Keyword,
    String,
    Number,
    Delim,
    Punct,
    /// Given valid code, then this variant should never be encountered. If
    /// is is encountered, then check if an issue has already been submitted,
    /// otherwise please [submit an issue].
    ///
    /// [submit an issue]: https://github.com/vallentin/colorblast/issues
    Invalid,
}

impl StylizeToken for Token {
    fn style(&self, _span: &TokenSpan<'_>) -> Style {
        match self {
            Self::Space => Style::new().fg((212, 212, 212)),
            Self::Comment => Style::new().fg((106, 153, 85)),
            Self::Ident => Style::new().fg((156, 220, 254)),
            Self::Keyword => Style::new().fg((86, 156, 214)),
            Self::String => Style::new().fg((206, 145, 120)),
            Self::Number => Style::new().fg((181, 206, 168)),
            Self::Delim | Self::Punct => Style::new().fg((212, 212, 212)),
            // #[cfg(not(debug_assertions))]
            // Self::Invalid => Style::new().fg((212, 212, 212)),
            // #[cfg(debug_assertions)]
            Self::Invalid => Style::new().fg((255, 0, 0)).bg((68, 17, 17)),
        }
    }
}
