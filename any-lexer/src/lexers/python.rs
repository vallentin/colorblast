use text_scanner::ext::{PythonScannerExt, PythonStrExt};
use text_scanner::Scanner;

use crate::{impl_lexer_from_scanner, ScanToken, ScannerExt, TokenSpan};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum PythonToken {
    Space,
    LineComment,
    ExplicitLineJoiner,
    Ident,
    Keyword,
    SoftKeyword,
    ShortString,
    LongString,
    ShortBytes,
    LongBytes,
    Int,
    Float,
    Delim,
    Punct,
    /// Given valid Python code, then this variant should never be encountered.
    /// If is is encountered, then check if an issue has already been submitted,
    /// otherwise please [submit an issue].
    ///
    /// [submit an issue]: https://github.com/vallentin/colorblast/issues
    Unknown,
}

impl ScanToken for PythonToken {
    #[inline]
    fn scan_token<'text>(scanner: &mut Scanner<'text>) -> Option<(Self, TokenSpan<'text>)> {
        let (r, _s) = scanner.skip_whitespace();
        if !r.is_empty() {
            return Some((Self::Space, scanner.span(r)));
        }

        if let Ok((r, _s)) = scanner.scan_python_line_comment() {
            return Some((Self::LineComment, scanner.span(r)));
        }

        if let Ok((r, ident)) = scanner.scan_python_identifier() {
            let tok = if ident.is_python_keyword() {
                Self::Keyword
            } else if ident.is_python_soft_keyword() {
                Self::SoftKeyword
            } else {
                Self::Ident
            };
            return Some((tok, scanner.span(r)));
        }

        if let Ok((r, _s)) = scanner.scan_python_long_string() {
            return Some((Self::LongString, scanner.span(r)));
        } else if let Ok((r, _s)) = scanner.scan_python_short_string() {
            return Some((Self::ShortString, scanner.span(r)));
        } else if let Ok((r, _s)) = scanner.scan_python_long_bytes() {
            return Some((Self::LongBytes, scanner.span(r)));
        } else if let Ok((r, _s)) = scanner.scan_python_short_bytes() {
            return Some((Self::ShortBytes, scanner.span(r)));
        }

        if let Ok((r, _s)) = scanner.scan_python_float() {
            return Some((Self::Float, scanner.span(r)));
        } else if let Ok((r, _s)) = scanner
            .scan_python_int_hex()
            .or_else(|_| scanner.scan_python_int_oct())
            .or_else(|_| scanner.scan_python_int_dec())
        {
            return Some((Self::Int, scanner.span(r)));
        }

        if let Ok((r, _c)) = scanner.scan_python_delimiter() {
            return Some((Self::Delim, scanner.span(r)));
        } else if let Ok((r, _c)) = scanner.scan_python_operator() {
            return Some((Self::Punct, scanner.span(r)));
        }

        if let Ok((r, _s)) = scanner.scan_python_explicit_line_joiner() {
            return Some((Self::ExplicitLineJoiner, scanner.span(r)));
        }

        let (r, _c) = scanner.next().ok()?;
        Some((Self::Unknown, scanner.span(r)))
    }
}

/// Python lexer producing [`PythonToken`]s.
///
/// **Note:** Cloning `PythonLexer` is essentially a copy, as it just contains
/// a `&str` and a `usize` for its `cursor`. However, `Copy` is not
/// implemented, to avoid accidentally copying immutable `PythonLexer`s.
#[derive(Clone, Debug)]
pub struct PythonLexer<'text> {
    scanner: Scanner<'text>,
}

impl<'text> PythonLexer<'text> {
    #[inline]
    pub fn new(text: &'text str) -> Self {
        Self {
            scanner: Scanner::new(text),
        }
    }
}

impl_lexer_from_scanner!('text, PythonLexer<'text>, PythonToken, scanner);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_lexer_spans() {
        // This intentionally uses Rust code as input, as it is
        // only testing that PythonLexer returns all characters
        let input = include_str!("../../../text-scanner/src/ext/rust.rs");
        let mut output = String::new();

        let lexer = PythonLexer::new(input);
        for (_tok, span) in lexer {
            output.push_str(span.as_str());
        }

        assert_eq!(input, output);
    }
}
