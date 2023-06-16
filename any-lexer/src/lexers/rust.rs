use std::iter::FusedIterator;

use text_scanner::{ext::RustScannerExt, Scanner};

use crate::{ScannerExt, TokenSpan};

#[rustfmt::skip]
const KEYWORDS: [&str; 53] = [
    "as", "break", "const", "continue", "crate", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "macro_rules",
    "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self",
    "static", "struct", "super", "trait", "true", "type", "union", "unsafe",
    "use", "where", "while", "async", "await", "dyn", "abstract", "become",
    "box", "do", "final", "macro", "override", "priv", "typeof", "unsized",
    "virtual", "yield", "try",
];

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RustToken {
    Space,
    LineComment,
    BlockComment,
    Ident,
    Keyword,
    Lifetime,
    Char,
    String,
    RawString,
    Int,
    Float,
    Delim,
    Punct,
    Unknown,
}

impl RustToken {
    pub fn scan<'text>(scanner: &mut Scanner<'text>) -> Option<(Self, TokenSpan<'text>)> {
        let (r, _s) = scanner.skip_whitespace();
        if !r.is_empty() {
            return Some((Self::Space, scanner.span(r)));
        }

        if let Ok((r, _s)) = scanner.scan_rust_line_comment() {
            return Some((Self::LineComment, scanner.span(r)));
        } else if let Ok((r, _s)) = scanner.scan_rust_block_comment() {
            return Some((Self::BlockComment, scanner.span(r)));
        }

        if let Ok((r, ident)) = scanner
            .scan_rust_raw_identifier()
            .or_else(|_| scanner.scan_rust_identifier())
        {
            let tok = if KEYWORDS.contains(&ident) {
                Self::Keyword
            } else {
                Self::Ident
            };
            return Some((tok, scanner.span(r)));
        }

        match scanner.peek() {
            Ok((_r, '\'')) => {
                if let Ok((r, _s)) = scanner.scan_rust_char() {
                    return Some((Self::Char, scanner.span(r)));
                }

                let res = scanner.scan_with(|scanner| {
                    scanner.accept_char('\'')?;
                    scanner.scan_rust_identifier()?;
                    Ok(())
                });
                if let Ok((r, _s)) = res {
                    return Some((Self::Lifetime, scanner.span(r)));
                }

                let (r, _c) = scanner.next().ok()?;
                return Some((Self::Unknown, scanner.span(r)));
            }
            _ => {}
        }

        if let Ok((r, _s)) = scanner
            .scan_rust_raw_string()
            .or_else(|_| scanner.scan_rust_string())
        {
            return Some((Self::String, scanner.span(r)));
        }

        if let Ok((r, _s)) = scanner.scan_rust_float() {
            return Some((Self::Float, scanner.span(r)));
        } else if let Ok((r, _s)) = scanner
            .scan_rust_int_hex()
            .or_else(|_| scanner.scan_rust_int_oct())
            .or_else(|_| scanner.scan_rust_int_bin())
            .or_else(|_| scanner.scan_rust_int_dec())
        {
            return Some((Self::Int, scanner.span(r)));
        }

        if let Ok((r, _c)) = scanner.accept_char_any(&['{', '}', '[', ']', '(', ')']) {
            return Some((Self::Delim, scanner.span(r)));
        }

        let res = scanner.scan_with(|scanner| {
            let (r, c) = scanner.next()?;
            match c {
                '=' => {
                    _ = scanner.accept_char_any(&['=', '>']);
                }
                '-' => {
                    _ = scanner.accept_char_any(&['=', '>']);
                }
                '+' | '*' | '/' | '%' | '^' | '!' => {
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
                    if scanner.accept_char('.').is_ok() {
                        _ = scanner.accept_char_any(&['.', '=']);
                    }
                }
                ':' => {
                    _ = scanner.accept_char(':');
                }
                '@' | '_' | ',' | ';' | '#' | '$' | '?' | '~' => {}
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
pub struct RustLexer<'text> {
    scanner: Scanner<'text>,
}

impl<'text> RustLexer<'text> {
    #[inline]
    pub fn new(text: &'text str) -> Self {
        Self {
            scanner: Scanner::new(text),
        }
    }
}

impl<'text> Iterator for RustLexer<'text> {
    type Item = (RustToken, TokenSpan<'text>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        RustToken::scan(&mut self.scanner)
    }
}

impl FusedIterator for RustLexer<'_> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_end_to_end() {
        let input = include_str!("../../../text-scanner/src/ext/rust.rs");
        let mut output = String::new();

        let lexer = RustLexer::new(input);
        for (_tok, span) in lexer {
            output.push_str(span.as_str());
        }

        assert_eq!(input, output);
    }
}
