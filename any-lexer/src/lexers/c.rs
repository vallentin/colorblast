use std::iter::FusedIterator;

use text_scanner::{ext::CScannerExt, Scanner};

use crate::{ScannerExt, TokenSpan};

// Reference: https://learn.microsoft.com/en-us/cpp/c-language/c-keywords?view=msvc-170#standard-c-keywords
#[rustfmt::skip]
const KEYWORDS: [&str; 46] = [
    "alignas", "alignof", "auto", "break", "case", "char", "const", "continue",
    "default", "do", "double", "else", "enum", "extern", "float", "for", "goto",
    "if", "inline", "int", "long", "register", "restrict", "return", "short",
    "signed", "sizeof", "static", "struct", "switch", "typedef", "union",
    "unsigned", "void", "volatile", "while", "_Alignas", "_Alignof", "_Atomic",
    "_Bool", "_Complex", "_Generic", "_Imaginary", "_Noreturn", "_Static_assert",
    "_Thread_local",
];

// Reference: https://learn.microsoft.com/en-us/cpp/c-language/c-keywords?view=msvc-170#microsoft-specific-c-keywords
#[rustfmt::skip]
const KEYWORDS_MS: [&str; 21] = [
    "__asm", "__based", "__cdecl", "__declspec", "__except", "__fastcall", "__finally",
    "__inline", "__int16", "__int32", "__int64", "__int8", "__leave", "__restrict",
    "__stdcall", "__try", "dllexport", "dllimport", "naked", "static_assert", "thread",
];

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CToken {
    Space,
    LineComment,
    BlockComment,
    Ident,
    Keyword,
    Char,
    String,
    Int,
    Float,
    Delim,
    Punct,
    /// Given valid C code, then this variant should never be encountered. If
    /// is is encountered, then check if an issue has already been submitted,
    /// otherwise please [submit an issue].
    ///
    /// [submit an issue]: https://github.com/vallentin/colorblast/issues
    Unknown,
}

impl CToken {
    pub fn scan<'text>(scanner: &mut Scanner<'text>) -> Option<(Self, TokenSpan<'text>)> {
        let (r, _s) = scanner.skip_whitespace();
        if !r.is_empty() {
            return Some((Self::Space, scanner.span(r)));
        }

        if let Ok((r, _s)) = scanner.scan_c_line_comment() {
            return Some((Self::LineComment, scanner.span(r)));
        } else if let Ok((r, _s)) = scanner.scan_c_block_comment() {
            return Some((Self::BlockComment, scanner.span(r)));
        }

        if let Ok((r, ident)) = scanner.scan_c_identifier() {
            let tok = if KEYWORDS.contains(&ident) {
                Self::Keyword
            } else if KEYWORDS_MS.contains(&ident) {
                Self::Keyword
            } else {
                Self::Ident
            };
            return Some((tok, scanner.span(r)));
        }

        if let Ok((r, _s)) = scanner.scan_c_char() {
            return Some((Self::Char, scanner.span(r)));
        } else if let Ok((r, _s)) = scanner.scan_c_string() {
            return Some((Self::String, scanner.span(r)));
        }

        if let Ok((r, _s)) = scanner.scan_c_float() {
            return Some((Self::Float, scanner.span(r)));
        } else if let Ok((r, _s)) = scanner
            .scan_c_int_hex()
            .or_else(|_| scanner.scan_c_int_oct())
            .or_else(|_| scanner.scan_c_int_dec())
        {
            return Some((Self::Int, scanner.span(r)));
        }

        if let Ok((r, _c)) = scanner.accept_char_any(&['{', '}', '[', ']', '(', ')']) {
            return Some((Self::Delim, scanner.span(r)));
        }

        // Reference: https://learn.microsoft.com/en-us/cpp/c-language/c-operators?view=msvc-170
        let res = scanner.scan_with(|scanner| {
            let (r, c) = scanner.next()?;
            match c {
                '=' => {
                    _ = scanner.accept_char_any(&['=', '>']);
                }
                '+' => {
                    _ = scanner.accept_char_any(&['+', '=']);
                }
                '-' => {
                    _ = scanner.accept_char_any(&['-', '=']);
                }
                '*' | '/' | '%' | '^' | '!' => {
                    _ = scanner.accept_char('=');
                }
                '&' => {
                    _ = scanner.accept_char_any(&['&', '=']);
                }
                '|' => {
                    _ = scanner.accept_char_any(&['|', '=']);
                }
                '<' => {
                    _ = scanner.accept_char('<');
                    _ = scanner.accept_char('=');
                }
                '>' => {
                    _ = scanner.accept_char('>');
                    _ = scanner.accept_char('=');
                }
                '.' => {
                    _ = scanner.scan_with(|scanner| {
                        scanner.accept_char('.')?;
                        scanner.accept_char('.')?;
                        Ok(())
                    });
                }
                '#' => {
                    _ = scanner.accept_char('#');
                }
                ',' | ';' | ':' | '?' | '~' => {}
                _ => return Err(scanner.ranged_text(r)),
            }
            Ok(())
        });
        if let Ok((r, _s)) = res {
            return Some((Self::Punct, scanner.span(r)));
        }

        let (r, _c) = scanner.next().ok()?;
        Some((Self::Unknown, scanner.span(r)))
    }
}

#[derive(Clone, Debug)]
pub struct CLexer<'text> {
    scanner: Scanner<'text>,
}

impl<'text> CLexer<'text> {
    #[inline]
    pub fn new(text: &'text str) -> Self {
        Self {
            scanner: Scanner::new(text),
        }
    }
}

impl<'text> Iterator for CLexer<'text> {
    type Item = (CToken, TokenSpan<'text>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        CToken::scan(&mut self.scanner)
    }
}

impl FusedIterator for CLexer<'_> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_lexer_spans() {
        // This intentionally uses Rust code as input, as it is
        // only testing that CLexer returns all characters
        let input = include_str!("../../../text-scanner/src/ext/rust.rs");
        let mut output = String::new();

        let lexer = CLexer::new(input);
        for (_tok, span) in lexer {
            output.push_str(span.as_str());
        }

        assert_eq!(input, output);
    }
}
