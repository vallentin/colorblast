use super::{impl_iter, LexerExt, Token, TokenSpan};

/// JSON lexer producing <code>([`Token`], [`TokenSpan`])</code>
/// for classifying JSON.
///
/// **Note:** Cloning `JsonLexer` is essentially a copy, as it just contains
/// a `&str` and a `usize` for its `cursor`. However, `Copy` is not
/// implemented, to avoid accidentally copying immutable `JsonLexer`s.
///
/// # Warning
///
/// If you are about to use `JsonLexer` for anything outside the scope of the
/// [`colorblast` crate], then please see the warning in the [`lexers` module].
///
/// [`colorblast` crate]: crate
/// [`lexers` module]: super#warning
#[derive(Clone, Debug)]
pub struct JsonLexer<'json> {
    lexer: any_lexer::JsonLexer<'json>,
}

impl<'json> JsonLexer<'json> {
    #[inline]
    pub fn new(json: &'json str) -> Self {
        Self {
            lexer: any_lexer::JsonLexer::new(json),
        }
    }

    fn next_token(&mut self) -> Option<(Token, TokenSpan<'json>)> {
        use any_lexer::JsonToken;
        let (tok, span) = self.lexer.next()?;
        let tok = match tok {
            JsonToken::Space => Token::Space,
            JsonToken::String => {
                let next_token = self
                    .lexer
                    .peek_find_token(|tok| !matches!(tok, JsonToken::Space));
                match next_token {
                    Some((JsonToken::Punct, span)) if span.as_str() == ":" => Token::Var,
                    _ => Token::String,
                }
            }
            JsonToken::Number => Token::Number,
            JsonToken::Null | JsonToken::True | JsonToken::False => Token::Keyword,
            JsonToken::Delim => Token::Delimiter,
            JsonToken::Punct => Token::Operator,
            JsonToken::Unknown => Token::Invalid,
        };
        Some((tok, span))
    }
}

impl_iter!('json, JsonLexer<'json>);
