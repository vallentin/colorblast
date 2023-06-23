use text_scanner::{ext::ScssScannerExt, Scanner};

use crate::{impl_lexer_from_scanner, CssToken, ScanToken, ScannerExt, TokenSpan};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ScssToken {
    Space,
    LineComment,
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
    /// Given valid SCSS, then this variant should never be encountered. If is
    /// is encountered, then check if an issue has already been submitted,
    /// otherwise please [submit an issue].
    ///
    /// [submit an issue]: https://github.com/vallentin/colorblast/issues
    Unknown,
}

impl ScanToken for ScssToken {
    fn scan_token<'text>(scanner: &mut Scanner<'text>) -> Option<(Self, TokenSpan<'text>)> {
        if let Ok((r, _s)) = scanner.scan_scss_line_comment() {
            return Some((Self::LineComment, scanner.span(r)));
        }

        let (tok, span) = CssToken::scan_token(scanner)?;
        match tok {
            CssToken::Space => Some((ScssToken::Space, span)),
            CssToken::BlockComment => Some((ScssToken::BlockComment, span)),
            CssToken::Ident => Some((ScssToken::Ident, span)),
            CssToken::AtKeyword => Some((ScssToken::AtKeyword, span)),
            CssToken::Hash => Some((ScssToken::Hash, span)),
            CssToken::String => Some((ScssToken::String, span)),
            CssToken::Number => Some((ScssToken::Number, span)),
            CssToken::Punct => Some((ScssToken::Punct, span)),
            CssToken::Delim => Some((ScssToken::Delim, span)),
            CssToken::Unknown => Some((ScssToken::Unknown, span)),
        }
    }
}

/// SCSS lexer producing [`ScssToken`]s.
///
/// **Note:** Cloning `ScssLexer` is essentially a copy, as it just contains
/// a `&str` and a `usize` for its `cursor`. However, `Copy` is not
/// implemented, to avoid accidentally copying immutable `ScssLexer`s.
///
/// See also [`CssLexer`].
///
/// [`CssLexer`]: super::CssLexer
#[derive(Clone, Debug)]
pub struct ScssLexer<'text> {
    scanner: Scanner<'text>,
}

impl<'text> ScssLexer<'text> {
    #[inline]
    pub fn new(text: &'text str) -> Self {
        Self {
            scanner: Scanner::new(text),
        }
    }
}

impl_lexer_from_scanner!('text, ScssLexer<'text>, ScssToken, scanner);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scss_lexer_spans() {
        // This intentionally uses Rust code as input, as it is
        // only testing that ScssLexer returns all characters
        let input = include_str!("../../../text-scanner/src/ext/rust.rs");
        let mut output = String::new();

        let lexer = ScssLexer::new(input);
        for (_tok, span) in lexer {
            output.push_str(span.as_str());
        }

        assert_eq!(input, output);
    }
}
