use text_scanner::{ext::CScannerExt, Scanner};

use crate::{impl_lexer_from_scanner, ScanToken, ScannerExt, TokenSpan};

// Reference: https://en.cppreference.com/w/cpp/keyword
#[rustfmt::skip]
const KEYWORDS: [&str; 97] = [
    "alignas", "alignof", "and", "and_eq", "asm", "atomic_cancel", "atomic_commit",
    "atomic_noexcept", "auto", "bitand", "bitor", "bool", "break", "case", "catch",
    "char", "char8_t", "char16_t", "char32_t", "class", "compl", "concept", "const",
    "consteval", "constexpr", "constinit", "const_cast", "continue", "co_await",
    "co_return", "co_yield", "decltype", "default", "delete", "do", "double", "dynamic_cast",
    "else", "enum", "explicit", "export", "extern", "false", "float", "for", "friend",
    "goto", "if", "inline", "int", "long", "mutable", "namespace", "new", "noexcept",
    "not", "not_eq", "nullptr", "operator", "or", "or_eq", "private", "protected",
    "public", "reflexpr", "register", "reinterpret_cast", "requires", "return",
    "short", "signed", "sizeof", "static", "static_assert", "static_cast", "struct",
    "switch", "synchronized", "template", "this", "thread_local", "throw", "true",
    "try", "typedef", "typeid", "typename", "union", "unsigned", "using", "virtual",
    "void", "volatile", "wchar_t", "while", "xor", "xor_eq",
];

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CppToken {
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
    /// Given valid C++ code, then this variant should never be encountered. If
    /// is is encountered, then check if an issue has already been submitted,
    /// otherwise please [submit an issue].
    ///
    /// [submit an issue]: https://github.com/vallentin/colorblast/issues
    Unknown,
}

impl ScanToken for CppToken {
    fn scan_token<'text>(scanner: &mut Scanner<'text>) -> Option<(Self, TokenSpan<'text>)> {
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

        // Reference: https://en.cppreference.com/w/cpp/language/punctuators
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
                    let res = scanner.accept_char_any(&['-', '=']);
                    if res.is_err() && scanner.accept_char('>').is_ok() {
                        let _ = scanner.accept_char('*');
                    }
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
                    let res1 = scanner.accept_char('<');
                    let res2 = scanner.accept_char('=');
                    if res1.is_ok() && res2.is_ok() {
                        _ = scanner.accept_char('>');
                    }
                }
                '>' => {
                    _ = scanner.accept_char('>');
                    _ = scanner.accept_char('=');
                }
                '.' => {
                    let res = scanner.accept_char('*');
                    if res.is_err() {
                        _ = scanner.scan_with(|scanner| {
                            scanner.accept_char('.')?;
                            scanner.accept_char('.')?;
                            Ok(())
                        });
                    }
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

/// C++ lexer producing [`CppToken`]s.
///
/// **Note:** Cloning `CppLexer` is essentially a copy, as it just contains
/// a `&str` and a `usize` for its `cursor`. However, `Copy` is not
/// implemented, to avoid accidentally copying immutable `CppLexer`s.
#[derive(Clone, Debug)]
pub struct CppLexer<'text> {
    scanner: Scanner<'text>,
}

impl<'text> CppLexer<'text> {
    #[inline]
    pub fn new(text: &'text str) -> Self {
        Self {
            scanner: Scanner::new(text),
        }
    }
}

impl_lexer_from_scanner!('text, CppLexer<'text>, CppToken, scanner);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpp_lexer_spans() {
        // This intentionally uses Rust code as input, as it is
        // only testing that CppLexer returns all characters
        let input = include_str!("../../../text-scanner/src/ext/rust.rs");
        let mut output = String::new();

        let lexer = CppLexer::new(input);
        for (_tok, span) in lexer {
            output.push_str(span.as_str());
        }

        assert_eq!(input, output);
    }
}
