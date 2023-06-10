use crate::{lexers::RustToken, Style, StylizeToken, TokenSpan};

impl StylizeToken for RustToken {
    fn style(&self, _span: &TokenSpan<'_>) -> Style {
        match self {
            Self::Space => Style::new().fg((212, 212, 212)),
            Self::LineComment | Self::BlockComment => Style::new().fg((106, 153, 85)),
            Self::Ident => Style::new().fg((156, 220, 254)),
            Self::Keyword => Style::new().fg((86, 156, 214)),
            Self::Lifetime => Style::new().fg((86, 156, 214)),
            Self::Char | Self::String | Self::RawString => Style::new().fg((206, 145, 120)),
            Self::Int | Self::Float => Style::new().fg((181, 206, 168)),
            Self::Delim | Self::Punct => Style::new().fg((212, 212, 212)),
            #[cfg(not(debug_assertions))]
            Self::Unknown => Style::new().fg((212, 212, 212)),
            #[cfg(debug_assertions)]
            Self::Unknown => Style::new().fg((255, 0, 0)).bg((68, 17, 17)),
        }
    }
}
