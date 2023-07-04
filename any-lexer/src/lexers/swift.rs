use text_scanner::ext::{SwiftScannerExt, SwiftStrExt};
use text_scanner::Scanner;

use crate::{impl_lexer_from_scanner, ScanToken, ScannerExt, TokenSpan};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SwiftToken {
    Space,
    LineComment,
    BlockComment,
    Ident,
    AttribName,
    Keyword,
    Punct,
    Delim,
    Nil,
    Boolean,
    Int,
    Float,
    String,
    Regex,
    /// Given valid Swift code, then this variant should never be encountered.
    /// If is is encountered, then check if an issue has already been submitted,
    /// otherwise please [submit an issue].
    ///
    /// [submit an issue]: https://github.com/vallentin/colorblast/issues
    Unknown,
}

impl ScanToken for SwiftToken {
    fn scan_token<'text>(scanner: &mut Scanner<'text>) -> Option<(Self, TokenSpan<'text>)> {
        let (r, _s) = scanner.skip_whitespace();
        if !r.is_empty() {
            return Some((Self::Space, scanner.span(r)));
        }

        if let Ok((r, _s)) = scanner.scan_swift_line_comment() {
            return Some((Self::LineComment, scanner.span(r)));
        } else if let Ok((r, _s)) = scanner.scan_swift_block_comment() {
            return Some((Self::BlockComment, scanner.span(r)));
        }

        if let Ok((r, ident)) = scanner.scan_swift_identifier() {
            let tok = match ident {
                _ if ident.is_swift_nil_literal() => Self::Nil,
                _ if ident.is_swift_boolean_literal() => Self::Boolean,
                _ if ident.is_swift_keyword() => Self::Keyword,
                _ => Self::Ident,
            };
            return Some((tok, scanner.span(r)));
        } else if let Ok((r, _c)) = scanner.scan_swift_attribute_name() {
            return Some((Self::AttribName, scanner.span(r)));
        }

        if let Ok((r, f)) = scanner.scan_swift_float_literal() {
            let tok = if f.contains('.') {
                Self::Float
            } else {
                Self::Int
            };
            return Some((tok, scanner.span(r)));
        } else if let Ok((r, _s)) = scanner.scan_swift_int_literal() {
            return Some((Self::Int, scanner.span(r)));
        }

        if let Ok((r, _s)) = scanner.scan_swift_string_literal() {
            return Some((Self::String, scanner.span(r)));
        } else if let Ok((r, _c)) = scanner.scan_swift_regex_literal() {
            return Some((Self::Regex, scanner.span(r)));
        }

        if let Ok((r, _c)) = scanner.scan_swift_operator() {
            return Some((Self::Punct, scanner.span(r)));
        } else if let Ok((r, _c)) = scanner.scan_swift_delimiter() {
            return Some((Self::Delim, scanner.span(r)));
        }

        let (r, _c) = scanner.next().ok()?;
        Some((Self::Unknown, scanner.span(r)))
    }
}

/// Swift lexer producing [`SwiftToken`]s.
///
/// **Note:** Cloning `SwiftLexer` is essentially a copy, as it just contains
/// a `&str` and a `usize` for its `cursor`. However, `Copy` is not
/// implemented, to avoid accidentally copying immutable `SwiftLexer`s.
#[derive(Clone, Debug)]
pub struct SwiftLexer<'text> {
    scanner: Scanner<'text>,
}

impl<'text> SwiftLexer<'text> {
    #[inline]
    pub fn new(text: &'text str) -> Self {
        Self {
            scanner: Scanner::new(text),
        }
    }
}

impl_lexer_from_scanner!('text, SwiftLexer<'text>, SwiftToken, scanner);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swift_lexer_spans() {
        // This intentionally uses Rust code as input, as it is
        // only testing that SwiftLexer returns all characters
        let input = include_str!("../../../text-scanner/src/ext/rust.rs");
        let mut output = String::new();

        let lexer = SwiftLexer::new(input);
        for (_tok, span) in lexer {
            output.push_str(span.as_str());
        }

        assert_eq!(input, output);
    }
}
