use text_scanner::{ext::JsonScannerExt, Scanner};

use crate::{impl_lexer_from_scanner, ScanToken, ScannerExt, TokenSpan};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum JsonToken {
    Space,
    String,
    Number,
    Null,
    True,
    False,
    /// Punctuation e.g. `:`, `,`.
    Punct,
    /// Delimiter e.g. `{`, `}`, `[`, and `]`.
    Delim,
    /// Given valid JSON, then this variant should never be encountered. If
    /// is is encountered, then check if an issue has already been submitted,
    /// otherwise please [submit an issue].
    ///
    /// [submit an issue]: https://github.com/vallentin/colorblast/issues
    Unknown,
}

impl ScanToken for JsonToken {
    fn scan_token<'text>(scanner: &mut Scanner<'text>) -> Option<(Self, TokenSpan<'text>)> {
        let (r, _s) = scanner.skip_whitespace();
        if !r.is_empty() {
            return Some((Self::Space, scanner.span(r)));
        }

        if let Ok((r, _c)) = scanner.accept_char_any(&['{', '}', '[', ']']) {
            return Some((Self::Delim, scanner.span(r)));
        } else if let Ok((r, _c)) = scanner.accept_char_any(&[':', ',']) {
            return Some((Self::Punct, scanner.span(r)));
        } else if let Ok((r, _c)) = scanner.scan_json_string() {
            return Some((Self::String, scanner.span(r)));
        } else if let Ok((r, _c)) = scanner.scan_json_number() {
            return Some((Self::Number, scanner.span(r)));
        }

        let backtrack = scanner.cursor_pos();
        let res = scanner.scan_with(|scanner| {
            scanner.accept_if(|c| c.is_ascii_alphabetic())?;
            scanner.skip_while(|c| c.is_ascii_alphabetic());

            Ok(())
        });
        if let Ok((r, s)) = res {
            match s {
                "null" => return Some((Self::Null, scanner.span(r))),
                "true" => return Some((Self::True, scanner.span(r))),
                "false" => return Some((Self::False, scanner.span(r))),
                _ => {
                    scanner.set_cursor_pos(backtrack);
                }
            }
        }

        let (r, _c) = scanner.next().ok()?;
        Some((Self::Unknown, scanner.span(r)))
    }
}

/// JSON lexer producing [`JsonToken`]s.
///
/// **Note:** Cloning `JsonLexer` is essentially a copy, as it just contains
/// a `&str` and a `usize` for its `cursor`. However, `Copy` is not
/// implemented, to avoid accidentally copying immutable `JsonLexer`s.
#[derive(Clone, Debug)]
pub struct JsonLexer<'text> {
    scanner: Scanner<'text>,
}

impl<'text> JsonLexer<'text> {
    #[inline]
    pub fn new(text: &'text str) -> Self {
        Self {
            scanner: Scanner::new(text),
        }
    }
}

impl_lexer_from_scanner!('text, JsonLexer<'text>, JsonToken, scanner);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_lexer_spans() {
        // This intentionally uses Rust code as input, as it is
        // only testing that JsonLexer returns all characters
        let input = include_str!("../../../text-scanner/src/ext/rust.rs");
        let mut output = String::new();

        let lexer = JsonLexer::new(input);
        for (_tok, span) in lexer {
            output.push_str(span.as_str());
        }

        assert_eq!(input, output);
    }
}
