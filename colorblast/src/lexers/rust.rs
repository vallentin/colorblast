use std::mem;

use super::{impl_iter, IntoSimpleToken, SimpleTokenIter, Token, TokenSpan};

const KEYWORDS_CONTROL_FLOW: &[&str] = &[
    "await", "break", "continue", "do", "else", "for", "if", "in", "loop", "match", "return",
    "try", "while", "yield",
];

const PRIMITIVE_TYPES: &[&str] = &[
    "bool", "char", "f32", "f64", "fn", "i8", "i16", "i32", "i64", "i128", "isize", "str", "u8",
    "u16", "u32", "u64", "u128", "unit", "usize",
];

const VARIANTS: &[&str] = &["Some", "None", "Ok", "Err"];

impl IntoSimpleToken for any_lexer::RustToken {
    #[inline]
    fn into_simple_token(self) -> Token {
        match self {
            Self::Space => Token::Space,
            Self::LineComment | Self::BlockComment => Token::Comment,
            Self::Ident => Token::Var,
            Self::Keyword => Token::Keyword,
            Self::Lifetime => Token::Var4,
            Self::Char | Self::String | Self::RawString => Token::String,
            Self::Int | Self::Float => Token::Number,
            Self::Delim => Token::Delimiter,
            Self::Punct => Token::Operator,
            Self::Unknown => Token::Invalid,
        }
    }
}

/// Rust lexer producing <code>([`Token`], [`TokenSpan`])</code>
/// for classifying Rust code.
///
/// **Note:** Cloning `RustLexer` is essentially a copy, as it mainly just
/// contains a `&str` and a `usize` for its `cursor`. However, `Copy` is not
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
    tokens: SimpleTokenIter<any_lexer::RustLexer<'code>>,
    state: RustLexerState,
}

impl<'code> RustLexer<'code> {
    #[inline]
    pub fn new(code: &'code str) -> Self {
        Self {
            tokens: any_lexer::RustLexer::new(code).into(),
            state: RustLexerState::None,
        }
    }

    fn next_token(&mut self) -> Option<(Token, TokenSpan<'code>)> {
        let (mut tok, mut span) = self.tokens.next()?;

        match tok {
            Token::Space | Token::Comment => return Some((tok, span)),
            _ => {}
        }

        match &mut self.state {
            RustLexerState::None => {}
            RustLexerState::InAttr(brackets) => match tok {
                Token::Delimiter if span.as_str() == "[" => {
                    *brackets += 1;
                }
                Token::Delimiter if span.as_str() == "]" => {
                    if *brackets > 0 {
                        *brackets -= 1;
                    }
                    if *brackets == 0 {
                        self.state = RustLexerState::None;
                    }
                }
                _ => {}
            },
            RustLexerState::InUse => {}
            RustLexerState::NextIsFnName
            | RustLexerState::NextIsMacroName
            | RustLexerState::NextIsModName => {
                let state = mem::replace(&mut self.state, RustLexerState::None);
                if tok == Token::Var {
                    let tok = match state {
                        RustLexerState::NextIsFnName => Token::Var2,
                        RustLexerState::NextIsMacroName => Token::Var4,
                        RustLexerState::NextIsModName => Token::Var3,
                        _ => unreachable!(),
                    };
                    return Some((tok, span));
                }
            }
        }

        match tok {
            Token::Var | Token::Operator | Token::Delimiter
                if matches!(self.state, RustLexerState::InAttr(_)) =>
            {
                tok = Token::Meta;
            }
            Token::Var if matches!(self.state, RustLexerState::InUse) => {
                if span.as_str().contains(char::is_uppercase)
                    || self
                        .tokens
                        .peek_non_space_simple_token_if(
                            |(tok, span)| matches!(tok, Token::Operator if span.as_str() == "::"),
                        )
                        .is_some()
                {
                    tok = Token::Var3;
                } else {
                    tok = Token::Var2;
                }
            }
            Token::Keyword if span.as_str() == "use" => {
                self.state = RustLexerState::InUse;
            }
            Token::Keyword if span.as_str() == "mod" => {
                self.state = RustLexerState::NextIsModName;
            }
            Token::Keyword if KEYWORDS_CONTROL_FLOW.contains(&span.as_str()) => {
                tok = Token::Keyword2;
            }
            Token::Var if PRIMITIVE_TYPES.contains(&span.as_str()) => {
                tok = Token::Var3;
            }
            Token::Var if VARIANTS.contains(&span.as_str()) => {
                tok = Token::Var5;
            }
            Token::Var | Token::Keyword => {
                if (tok == Token::Keyword) && (span.as_str() == "fn") {
                    self.state = RustLexerState::NextIsFnName;
                }

                if (tok == Token::Var) && !span.as_str().contains(char::is_lowercase) {
                    tok = Token::Var5;
                } else if (tok == Token::Var)
                    && span
                        .as_str()
                        .chars()
                        .next()
                        .map_or(false, |c| c.is_uppercase())
                {
                    tok = Token::Var3;
                } else if let Some((_, next_span)) = self.tokens.next_non_space_simple_token_if(
                    |(tok, span)| matches!(tok, Token::Operator if span.as_str() == "!"),
                ) {
                    tok = Token::Var4;
                    span = span.join_unchecked(&next_span);

                    self.state = RustLexerState::NextIsMacroName;
                } else if tok == Token::Var {
                    let next_token =
                        self.tokens
                            .peek_non_space_simple_token_if(|(tok, span)| match tok {
                                Token::Operator if span.as_str() == "::" => true,
                                Token::Delimiter if span.as_str() == "(" => true,
                                _ => false,
                            });
                    if let Some((next_tok, _next_span)) = next_token {
                        match next_tok {
                            Token::Operator => tok = Token::Var3,
                            Token::Delimiter => tok = Token::Var2,
                            _ => unreachable!(),
                        }
                    }
                }
            }
            Token::Operator if span.as_str() == "#" => {
                self.state = RustLexerState::InAttr(0);
                tok = Token::Meta;
            }
            Token::Operator if (span.as_str() == ";") && (self.state == RustLexerState::InUse) => {
                self.state = RustLexerState::None;
            }
            _ => {}
        }

        Some((tok, span))
    }
}

impl_iter!('code, RustLexer<'code>);

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum RustLexerState {
    None,
    InAttr(usize),
    InUse,
    NextIsFnName,
    NextIsMacroName,
    NextIsModName,
}
