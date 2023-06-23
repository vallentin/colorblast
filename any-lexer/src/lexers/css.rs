use text_scanner::{ext::CssScannerExt, Scanner};

use crate::{impl_lexer_from_scanner, ScanToken, ScannerExt, TokenSpan};

const DELIMITERS: [char; 6] = ['{', '}', '[', ']', '(', ')'];
const PUNCTUATIONS: [char; 12] = [',', '.', ';', ':', '-', '+', '*', '=', '#', '!', '@', '%'];

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CssToken {
    Space,
    BlockComment,
    Ident,
    AtKeyword,
    Hash,
    String,
    Number,
    /// Punctuation e.g. `:`, `,`.
    Punct,
    /// Delimiter e.g. `{`, `}`, `[`, and `]`.
    Delim,
    /// Given valid CSS, then this variant should never be encountered. If
    /// is is encountered, then check if an issue has already been submitted,
    /// otherwise please [submit an issue].
    ///
    /// [submit an issue]: https://github.com/vallentin/colorblast/issues
    Unknown,
}

impl ScanToken for CssToken {
    fn scan_token<'text>(scanner: &mut Scanner<'text>) -> Option<(Self, TokenSpan<'text>)> {
        let (r, _s) = scanner.skip_whitespace();
        if !r.is_empty() {
            return Some((Self::Space, scanner.span(r)));
        }

        if let Ok((r, _c)) = scanner.scan_css_identifier() {
            return Some((Self::Ident, scanner.span(r)));
        } else if let Ok((r, _c)) = scanner.scan_css_at_keyword() {
            return Some((Self::AtKeyword, scanner.span(r)));
        } else if let Ok((r, _c)) = scanner.scan_css_hash() {
            return Some((Self::Hash, scanner.span(r)));
        }

        if let Ok((r, _c)) = scanner.scan_css_number() {
            return Some((Self::Number, scanner.span(r)));
        } else if let Ok((r, _c)) = scanner.scan_css_string() {
            return Some((Self::String, scanner.span(r)));
        }

        if let Ok((r, _c)) = scanner.accept_char_any(&DELIMITERS) {
            return Some((Self::Delim, scanner.span(r)));
        } else if let Ok((r, _c)) = scanner.accept_char_any(&PUNCTUATIONS) {
            return Some((Self::Punct, scanner.span(r)));
        }

        if let Ok((r, _c)) = scanner.scan_css_block_comment() {
            return Some((Self::BlockComment, scanner.span(r)));
        }

        let (r, _c) = scanner.next().ok()?;
        Some((Self::Unknown, scanner.span(r)))
    }
}

/// CSS lexer producing [`CssToken`]s.
///
/// **Note:** Cloning `CssLexer` is essentially a copy, as it just contains
/// a `&str` and a `usize` for its `cursor`. However, `Copy` is not
/// implemented, to avoid accidentally copying immutable `CssLexer`s.
///
/// See also [`ScssLexer`].
///
/// [`ScssLexer`]: super::ScssLexer
#[derive(Clone, Debug)]
pub struct CssLexer<'text> {
    scanner: Scanner<'text>,
}

impl<'text> CssLexer<'text> {
    #[inline]
    pub fn new(text: &'text str) -> Self {
        Self {
            scanner: Scanner::new(text),
        }
    }
}

impl_lexer_from_scanner!('text, CssLexer<'text>, CssToken, scanner);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_lexer_spans() {
        // This intentionally uses Rust code as input, as it is
        // only testing that CssLexer returns all characters
        let input = include_str!("../../../text-scanner/src/ext/rust.rs");
        let mut output = String::new();

        let lexer = CssLexer::new(input);
        for (_tok, span) in lexer {
            output.push_str(span.as_str());
        }

        assert_eq!(input, output);
    }
}
