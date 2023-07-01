use super::{impl_iter, Token, TokenSpan};

/// Rust lexer producing <code>([`Token`], [`TokenSpan`])</code>
/// for classifying Rust code.
///
/// **Note:** Cloning `RustLexer` is essentially a copy, as it just contains
/// a `&str` and a `usize` for its `cursor`. However, `Copy` is not
/// implemented, to avoid accidentally copying immutable `RustLexer`s.
///
/// # Warning
///
/// If you are about to use `RustLexer` for anything outside the scope of the
/// [`colorblast` crate], then please see the warning in the [`lexers` module].
///
/// [`colorblast` crate]: crate
/// [`lexers` module]: super#warning
#[derive(Clone, Debug)]
pub struct RustLexer<'code> {
    lexer: any_lexer::RustLexer<'code>,
}

impl<'code> RustLexer<'code> {
    #[inline]
    pub fn new(code: &'code str) -> Self {
        Self {
            lexer: any_lexer::RustLexer::new(code),
        }
    }

    fn next_token(&mut self) -> Option<(Token, TokenSpan<'code>)> {
        use any_lexer::RustToken;
        let (tok, span) = self.lexer.next()?;
        let tok = match tok {
            RustToken::Space => Token::Space,
            RustToken::LineComment | RustToken::BlockComment => Token::Comment,
            RustToken::Ident => Token::Var,
            RustToken::Keyword => Token::Keyword,
            RustToken::Lifetime => Token::Var4,
            RustToken::Char | RustToken::String | RustToken::RawString => Token::String,
            RustToken::Int | RustToken::Float => Token::Number,
            RustToken::Delim => Token::Delimiter,
            RustToken::Punct => Token::Operator,
            RustToken::Unknown => Token::Invalid,
        };
        Some((tok, span))
    }
}

impl_iter!('code, RustLexer<'code>);
