use text_scanner::ext::{LispLikeScannerExt, LispLikeToken as LispLikeTok};
use text_scanner::Scanner;

use crate::{impl_lexer_from_scanner, ScanToken, ScannerExt, TokenSpan};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum LispLikeToken {
    Space,
    SymbolName,
    Delim,
    Int,
    Float,
    Ratio,
    String,
    /// Given valid Lisp-like code, then this variant should never be encountered.
    /// If is is encountered, then check if an issue has already been submitted,
    /// otherwise please [submit an issue].
    ///
    /// [submit an issue]: https://github.com/vallentin/colorblast/issues
    Unknown,
}

impl ScanToken for LispLikeToken {
    fn scan_token<'text>(scanner: &mut Scanner<'text>) -> Option<(Self, TokenSpan<'text>)> {
        let (r, _s) = scanner.skip_whitespace();
        if !r.is_empty() {
            return Some((Self::Space, scanner.span(r)));
        }

        if let Ok(((r, _s), tok)) = scanner.scan_lisp_like_token() {
            return match tok {
                LispLikeTok::Symbol => Some((Self::SymbolName, scanner.span(r))),
                LispLikeTok::Int => Some((Self::Int, scanner.span(r))),
                LispLikeTok::Float => Some((Self::Float, scanner.span(r))),
                LispLikeTok::Ratio => Some((Self::Ratio, scanner.span(r))),
                LispLikeTok::String => Some((Self::String, scanner.span(r))),
            };
        }

        if let Ok((r, _c)) = scanner.scan_lisp_like_delimiter() {
            return Some((Self::Delim, scanner.span(r)));
        }

        let (r, _c) = scanner.next().ok()?;
        Some((Self::Unknown, scanner.span(r)))
    }
}

/// LispLike lexer producing [`LispLikeToken`]s.
///
/// **Note:** Cloning `LispLikeLexer` is essentially a copy, as it just contains
/// a `&str` and a `usize` for its `cursor`. However, `Copy` is not
/// implemented, to avoid accidentally copying immutable `LispLikeLexer`s.
#[derive(Clone, Debug)]
pub struct LispLikeLexer<'text> {
    scanner: Scanner<'text>,
}

impl<'text> LispLikeLexer<'text> {
    #[inline]
    pub fn new(text: &'text str) -> Self {
        Self {
            scanner: Scanner::new(text),
        }
    }
}

impl_lexer_from_scanner!('text, LispLikeLexer<'text>, LispLikeToken, scanner);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lisp_like_lexer_spans() {
        // This intentionally uses Rust code as input, as it is
        // only testing that LispLikeLexer returns all characters
        let input = include_str!("../../../text-scanner/src/ext/rust.rs");
        let mut output = String::new();

        let lexer = LispLikeLexer::new(input);
        for (_tok, span) in lexer {
            output.push_str(span.as_str());
        }

        assert_eq!(input, output);
    }
}
