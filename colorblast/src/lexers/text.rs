use super::{impl_iter, Token, TokenSpan};

/// Plain text lexer is a simple dummy passthrough tokenizer,
/// which produces at most a single <code>[Token]::[Text]</code>.
///
/// **Note:** Cloning `PlainTextLexer` is essentially a copy, as it
/// just contains a `&str`. However, `Copy` is not implemented,
/// to avoid accidentally copying immutable `PlainTextLexer`s.
///
/// # Warning
///
/// If you are about to use `PlainTextLexer` for anything outside the scope of the
/// [`colorblast` crate], then please see the warning in the [`lexers` module].
///
/// [`colorblast` crate]: crate
/// [`lexers` module]: super#warning
/// [Text]: Token::Text
#[derive(Clone, Debug)]
pub struct PlainTextLexer<'text> {
    text: Option<&'text str>,
}

impl<'text> PlainTextLexer<'text> {
    #[inline]
    pub fn new(text: &'text str) -> Self {
        Self { text: Some(text) }
    }

    #[inline]
    fn next_token(&mut self) -> Option<(Token, TokenSpan<'text>)> {
        let text = self.text.take()?;
        if text.is_empty() {
            return None;
        }
        Some((Token::Text, TokenSpan::new(text, 0..text.len())))
    }
}

impl_iter!('text, PlainTextLexer<'text>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_text_lexer_spans() {
        let input = include_str!("../../../text-scanner/src/ext/rust.rs");
        let mut output = String::new();

        let lexer = PlainTextLexer::new(input);
        for (_tok, span) in lexer {
            output.push_str(span.as_str());
        }

        assert_eq!(input, output);
    }
}
