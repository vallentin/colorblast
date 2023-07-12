pub mod prelude {
    pub use any_lexer::TokenSpan;

    pub use super::Token;
}

pub use any_lexer::TokenSpan;

use std::iter::FusedIterator;

use crate::style::Style;
use crate::stylize::StylizeToken;

macro_rules! impl_enum_token {
    (
        $(
            $(#[$attr:meta])*
            $name:ident,
        )+
    ) => {
        /// Generic `Token` with variants useful for applying syntax
        /// highlighting to [`TokenSpan`]s.
        ///
        /// **Warning:** `Token` is not intended for being used to build parsers.
        /// Instead use [`any-lexer`], which implements more specific lexers and
        /// tokens, such as:
        ///
        /// - [`RustLexer`] which produces [`RustToken`]s
        /// - [`CppLexer`] which produces [`CppToken`]s
        /// - [`PythonLexer`] which produces [`PythonToken`]s
        /// - [`ScssLexer`] which produces [`ScssToken`]s
        /// - _[and many more]_
        ///
        /// **Warning:** The variants of this enum is currently highly
        /// unstable. They will remain as such, until support for more
        /// languages has been implemented.
        ///
        /// [`any-lexer`]: https://crates.io/crates/any-lexer
        /// [`RustLexer`]: https://docs.rs/any-lexer/*/any_lexer/struct.RustLexer.html
        /// [`RustToken`]: https://docs.rs/any-lexer/*/any_lexer/enum.RustToken.html
        /// [`CppLexer`]: https://docs.rs/any-lexer/*/any_lexer/struct.CppLexer.html
        /// [`CppToken`]: https://docs.rs/any-lexer/*/any_lexer/enum.CppToken.html
        /// [`PythonLexer`]: https://docs.rs/any-lexer/*/any_lexer/struct.PythonLexer.html
        /// [`PythonToken`]: https://docs.rs/any-lexer/*/any_lexer/enum.PythonToken.html
        /// [`ScssLexer`]: https://docs.rs/any-lexer/*/any_lexer/struct.ScssLexer.html
        /// [`ScssToken`]: https://docs.rs/any-lexer/*/any_lexer/enum.ScssToken.html
        /// [and many more]: https://docs.rs/any-lexer/*/any_lexer/
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        #[non_exhaustive]
        pub enum Token {
            $(
                $(#[$attr])*
                $name,
            )+
        }

        impl Token {
            pub const VARIANTS: &[Self] = &[
                $(Self::$name,)+
            ];
        }
    };
}

impl_enum_token!(
    Space,
    Comment,
    /// Token representing text, which might contain whitespace.
    Text,
    Var,
    Var2,
    Var3,
    Var4,
    Var5,
    Keyword,
    Keyword2,
    Operator,
    Delimiter,
    Number,
    String,
    Meta,
    /// Given valid code, then this variant should never be encountered. If
    /// is is encountered, then check if an issue has already been submitted,
    /// otherwise please [submit an issue].
    ///
    /// [submit an issue]: https://github.com/vallentin/colorblast/issues
    Invalid,
);

impl StylizeToken for Token {
    fn style(&self, _span: &TokenSpan<'_>) -> Style {
        match self {
            // TODO: Background: (30, 30, 30)
            Self::Space => Style::new().fg((212, 212, 212)),
            Self::Comment => Style::new().fg((106, 153, 85)),
            Self::Text => Style::new().fg((212, 212, 212)),
            Self::Var => Style::new().fg((156, 220, 254)),
            Self::Var2 => Style::new().fg((220, 220, 170)),
            Self::Var3 => Style::new().fg((78, 201, 176)),
            Self::Var4 => Style::new().fg((86, 156, 214)),
            Self::Var5 => Style::new().fg((79, 193, 255)),
            Self::Keyword => Style::new().fg((86, 156, 214)),
            Self::Keyword2 => Style::new().fg((197, 134, 192)),
            Self::Operator => Style::new().fg((212, 212, 212)),
            Self::Delimiter => Style::new().fg((212, 212, 212)),
            Self::Number => Style::new().fg((181, 206, 168)),
            Self::String => Style::new().fg((206, 145, 120)),
            Self::Meta => Style::new().fg((212, 212, 212)),
            // #[cfg(not(debug_assertions))]
            // Self::Invalid => Style::new().fg((212, 212, 212)),
            // #[cfg(debug_assertions)]
            Self::Invalid => Style::new().fg((255, 0, 0)).bg((68, 17, 17)),
        }
    }
}

pub(crate) trait IntoSimpleToken {
    fn into_simple_token(self) -> Token;
}

#[derive(Clone, Debug)]
pub(crate) struct SimpleTokenIter<I> {
    iter: I,
}

impl<'text, T, I> SimpleTokenIter<I>
where
    I: Iterator<Item = (T, TokenSpan<'text>)>,
    T: IntoSimpleToken,
{
    #[inline]
    pub fn new(iter: I) -> Self {
        Self { iter }
    }

    #[inline]
    pub fn next_simple_token(&mut self) -> Option<(Token, TokenSpan<'text>)> {
        self.iter
            .next()
            .map(|(tok, span)| (tok.into_simple_token(), span))
    }

    /*
    #[inline]
    pub fn peek_simple_token(&self) -> Option<(Token, TokenSpan<'text>)>
    where
        I: Clone,
    {
        self.clone().next()
    }
    */

    #[inline]
    pub fn next_non_space_simple_token(&mut self) -> Option<(Token, TokenSpan<'text>)> {
        while let Some((tok, span)) = self.next_simple_token() {
            match tok {
                Token::Space => {}
                _ => return Some((tok, span)),
            }
        }
        None
    }

    #[inline]
    pub fn next_non_space_simple_token_if<F>(&mut self, f: F) -> Option<(Token, TokenSpan<'text>)>
    where
        I: Clone,
        F: FnOnce((Token, &TokenSpan<'text>)) -> bool,
    {
        let mut tokens = self.clone();
        let (tok, span) = tokens.next_non_space_simple_token()?;

        if f((tok, &span)) {
            self.iter = tokens.iter;
            Some((tok, span))
        } else {
            None
        }
    }

    #[inline]
    pub fn peek_non_space_simple_token(&self) -> Option<(Token, TokenSpan<'text>)>
    where
        I: Clone,
    {
        self.clone().next_non_space_simple_token()
    }

    #[inline]
    pub fn peek_non_space_simple_token_if<F>(&self, f: F) -> Option<(Token, TokenSpan<'text>)>
    where
        I: Clone,
        F: FnOnce((Token, &TokenSpan<'text>)) -> bool,
    {
        self.peek_non_space_simple_token()
            .filter(|(tok, span)| f((*tok, span)))
    }
}

impl<'text, T, I> Iterator for SimpleTokenIter<I>
where
    I: Iterator<Item = (T, TokenSpan<'text>)>,
    T: IntoSimpleToken,
{
    type Item = (Token, TokenSpan<'text>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.next_simple_token()
    }
}

impl<'text, T, I> FusedIterator for SimpleTokenIter<I>
where
    I: Iterator<Item = (T, TokenSpan<'text>)>,
    T: IntoSimpleToken,
{
}

impl<'text, T, I> From<I> for SimpleTokenIter<I>
where
    I: Iterator<Item = (T, TokenSpan<'text>)>,
    T: IntoSimpleToken,
{
    #[inline]
    fn from(iter: I) -> Self {
        Self::new(iter)
    }
}
