use super::{impl_iter, LexerExt, Token, TokenSpan};

/// [JSON with Comments] lexer producing <code>([`Token`], [`TokenSpan`])</code>
/// for classifying JSON with Comments.
///
/// **Note:** Cloning `JsonCLexer` is essentially a copy, as it just contains
/// a `&str` and a `usize` for its `cursor`. However, `Copy` is not
/// implemented, to avoid accidentally copying immutable `JsonCLexer`s.
///
/// # Warning
///
/// If you are about to use `JsonCLexer` for anything outside the scope of the
/// [`colorblast` crate], then please see the warning in the [`lexers` module].
///
/// [`colorblast` crate]: crate
/// [`lexers` module]: super#warning
/// [JSON with Comments]: https://code.visualstudio.com/docs/languages/json#_json-with-comments
#[derive(Clone, Debug)]
pub struct JsonCLexer<'jsonc> {
    lexer: any_lexer::JsonCLexer<'jsonc>,
}

impl<'jsonc> JsonCLexer<'jsonc> {
    #[inline]
    pub fn new(json: &'jsonc str) -> Self {
        Self {
            lexer: any_lexer::JsonCLexer::new(json),
        }
    }

    fn next_token(&mut self) -> Option<(Token, TokenSpan<'jsonc>)> {
        use any_lexer::JsonCToken;
        let (tok, span) = self.lexer.next()?;
        let tok = match tok {
            JsonCToken::Space => Token::Space,
            JsonCToken::LineComment => Token::Comment,
            JsonCToken::BlockComment => Token::Comment,
            JsonCToken::String => {
                let next_token = self
                    .lexer
                    .peek_find_token(|tok| !matches!(tok, JsonCToken::Space));
                match next_token {
                    Some((JsonCToken::Punct, span)) if span.as_str() == ":" => Token::Ident,
                    _ => Token::String,
                }
            }
            JsonCToken::Number => Token::Number,
            JsonCToken::Null | JsonCToken::True | JsonCToken::False => Token::Keyword,
            JsonCToken::Delim => Token::Delim,
            JsonCToken::Punct => Token::Punct,
            JsonCToken::Unknown => Token::Invalid,
        };
        Some((tok, span))
    }
}

impl_iter!('jsonc, JsonCLexer<'jsonc>);
