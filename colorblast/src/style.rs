pub mod prelude {
    pub use super::{AsStyle, Color, Style};
}

use std::mem;

#[allow(clippy::wrong_self_convention)]
pub trait AsStyle {
    fn as_style(self) -> Option<Style>;
}

impl AsStyle for Style {
    #[inline]
    fn as_style(self) -> Option<Style> {
        Some(self)
    }
}

impl AsStyle for Option<Style> {
    #[inline]
    fn as_style(self) -> Option<Style> {
        self
    }
}

impl AsStyle for Option<&Style> {
    #[inline]
    fn as_style(self) -> Option<Style> {
        self.cloned()
    }
}

impl AsStyle for Color {
    #[inline]
    fn as_style(self) -> Option<Style> {
        Some(Style::new().fg(self))
    }
}

impl AsStyle for Option<Color> {
    #[inline]
    fn as_style(self) -> Option<Style> {
        self.and_then(AsStyle::as_style)
    }
}

impl AsStyle for Option<&Color> {
    #[inline]
    fn as_style(self) -> Option<Style> {
        self.copied().and_then(AsStyle::as_style)
    }
}

impl AsStyle for [u8; 4] {
    #[inline]
    fn as_style(self) -> Option<Style> {
        Some(Style::new().fg(self))
    }
}

impl AsStyle for [u8; 3] {
    #[inline]
    fn as_style(self) -> Option<Style> {
        Some(Style::new().fg(self))
    }
}

impl AsStyle for Option<[u8; 4]> {
    #[inline]
    fn as_style(self) -> Option<Style> {
        self.and_then(AsStyle::as_style)
    }
}

impl AsStyle for Option<[u8; 3]> {
    #[inline]
    fn as_style(self) -> Option<Style> {
        self.and_then(AsStyle::as_style)
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
}

impl Default for Style {
    #[inline]
    fn default() -> Self {
        Self::NONE
    }
}

impl Style {
    pub const NONE: Self = Self { fg: None, bg: None };

    #[inline]
    pub fn new() -> Self {
        Self::NONE
    }

    #[inline]
    pub fn fg(mut self, color: impl Into<Color>) -> Self {
        self.fg = Some(color.into());
        self
    }

    #[inline]
    pub fn bg(mut self, color: impl Into<Color>) -> Self {
        self.bg = Some(color.into());
        self
    }

    #[inline]
    pub fn invert(mut self) -> Self {
        mem::swap(&mut self.fg, &mut self.bg);
        self
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Color(pub [u8; 4]);

impl Color {
    pub const BLACK: Self = Self([0x00, 0x00, 0x00, 0xFF]);
    pub const WHITE: Self = Self([0xFF, 0xFF, 0xFF, 0xFF]);
}

impl From<[u8; 4]> for Color {
    #[inline]
    fn from(rgba: [u8; 4]) -> Self {
        Self(rgba)
    }
}

impl From<[u8; 3]> for Color {
    #[inline]
    fn from([r, g, b]: [u8; 3]) -> Self {
        Self([r, g, b, 255])
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    #[inline]
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self([r, g, b, a])
    }
}

impl From<(u8, u8, u8)> for Color {
    #[inline]
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self([r, g, b, 255])
    }
}
