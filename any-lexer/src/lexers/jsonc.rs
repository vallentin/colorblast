use text_scanner::{ext::JsonCScannerExt, Scanner};

use crate::{impl_lexer_from_scanner, JsonToken, ScanToken, ScannerExt, TokenSpan};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum JsonCToken {
    Space,
    LineComment,
    BlockComment,
    String,
    Number,
    Null,
    True,
    False,
    /// Punctuation e.g. `:`, `,`.
    Punct,
    /// Delimiter e.g. `{`, `}`, `[`, and `]`.
    Delim,
    /// Given valid JSON with Comments, then this variant should never be
    /// encountered. If is is encountered, then check if an issue has already
    /// been submitted, otherwise please [submit an issue].
    ///
    /// [submit an issue]: https://github.com/vallentin/colorblast/issues
    Unknown,
}

impl ScanToken for JsonCToken {
    fn scan_token<'text>(scanner: &mut Scanner<'text>) -> Option<(Self, TokenSpan<'text>)> {
        if let Ok((r, _s)) = scanner.scan_jsonc_line_comment() {
            return Some((Self::LineComment, scanner.span(r)));
        } else if let Ok((r, _s)) = scanner.scan_jsonc_block_comment() {
            return Some((Self::BlockComment, scanner.span(r)));
        }

        let (tok, span) = JsonToken::scan_token(scanner)?;
        match tok {
            JsonToken::Space => Some((JsonCToken::Space, span)),
            JsonToken::String => Some((JsonCToken::String, span)),
            JsonToken::Number => Some((JsonCToken::Number, span)),
            JsonToken::Null => Some((JsonCToken::Null, span)),
            JsonToken::True => Some((JsonCToken::True, span)),
            JsonToken::False => Some((JsonCToken::False, span)),
            JsonToken::Punct => Some((JsonCToken::Punct, span)),
            JsonToken::Delim => Some((JsonCToken::Delim, span)),
            JsonToken::Unknown => Some((JsonCToken::Unknown, span)),
        }
    }
}

/// [JSON with Comments] lexer producing [`JsonCToken`]s.
///
/// **Note:** Cloning `JsonCLexer` is essentially a copy, as it just contains
/// a `&str` and a `usize` for its `cursor`. However, `Copy` is not
/// implemented, to avoid accidentally copying immutable `JsonCLexer`s.
///
/// [JSON with Comments]: https://code.visualstudio.com/docs/languages/json#_json-with-comments
#[derive(Clone, Debug)]
pub struct JsonCLexer<'text> {
    scanner: Scanner<'text>,
}

impl<'text> JsonCLexer<'text> {
    #[inline]
    pub fn new(text: &'text str) -> Self {
        Self {
            scanner: Scanner::new(text),
        }
    }
}

impl_lexer_from_scanner!('text, JsonCLexer<'text>, JsonCToken, scanner);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jsonc_lexer_spans() {
        // This intentionally uses Rust code as input, as it is
        // only testing that JsonCLexer returns all characters
        let input = include_str!("../../../text-scanner/src/ext/rust.rs");
        let mut output = String::new();

        let lexer = JsonCLexer::new(input);
        for (_tok, span) in lexer {
            output.push_str(span.as_str());
        }

        assert_eq!(input, output);
    }
}
