use crate::{CharExt, ScanResult, Scanner, ScannerResult};

// Reference: https://docs.python.org/3/reference/lexical_analysis.html#keywords
pub const PYTHON_KEYWORDS: &'static [&'static str] = &[
    "False", "await", "else", "import", "pass", "None", "break", "except", "in", "raise", "True",
    "class", "finally", "is", "return", "and", "continue", "for", "lambda", "try", "as", "def",
    "from", "nonlocal", "while", "assert", "del", "global", "not", "with", "async", "elif", "if",
    "or", "yield",
];

// Reference: https://docs.python.org/3/reference/lexical_analysis.html#soft-keywords
pub const PYTHON_SOFT_KEYWORDS: &'static [&'static str] = &["match", "case", "_"];

// Mix between Python operators and delimiters.
//
// Reference: https://docs.python.org/3/reference/lexical_analysis.html#operators
// Reference: https://docs.python.org/3/reference/lexical_analysis.html#delimiters
pub const PYTHON_OPERATORS: &'static [&'static str] = &[
    "+", "-", "*", "**", "/", "//", "%", "@", "<<", ">>", "&", "|", "^", "~", ":=", "<", ">", "<=",
    ">=", "==", "!=", ",", ":", ".", ";", /*"@",*/ "=", "->", "+=", "-=", "*=", "/=", "//=",
    "%=", "@=", "&=", "|=", "^=", ">>=", "<<=", "**=",
];

// Reference: https://docs.python.org/3/reference/lexical_analysis.html#delimiters
pub const PYTHON_DELIMITERS: &'static [&'static str] = &["(", ")", "[", "]", "{", "}"];

/// [`Scanner`] extension for scanning Python tokens.
///
/// _Based on Python 3.11._
pub trait PythonScannerExt<'text>: crate::private::Sealed {
    fn scan_python_line_comment(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_python_explicit_line_joiner(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_python_identifier(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_python_keyword(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_python_soft_keyword(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_python_operator(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_python_delimiter(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_python_int_dec(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_python_int_hex(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_python_int_oct(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_python_int_bin(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_python_float(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_python_string(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_python_short_string(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_python_long_string(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_python_bytes(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_python_short_bytes(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_python_long_bytes(&mut self) -> ScannerResult<'text, &'text str>;
}

impl<'text> PythonScannerExt<'text> for Scanner<'text> {
    // Reference: https://docs.python.org/3/reference/lexical_analysis.html#comments
    fn scan_python_line_comment(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('#')?;
            scanner.skip_until_char_any(&['\n', '\r']);
            Ok(())
        })
    }

    // Reference: https://docs.python.org/3/reference/lexical_analysis.html#explicit-line-joining
    fn scan_python_explicit_line_joiner(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            let (r, _c) = scanner.accept_char('\\')?;

            if !scanner.has_remaining_text() {
                return Ok(());
            }

            let remaining = scanner.remaining_text();
            if remaining.starts_with('\n') || remaining.starts_with("\r\n") || (remaining == "\r") {
                return Ok(());
            }

            Err(scanner.ranged_text(r))
        })
    }

    // Reference: https://docs.python.org/3/reference/lexical_analysis.html#identifiers
    fn scan_python_identifier(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_if(|c| c.is_alphabetic() || (c == '_'))?;
            scanner.skip_while(|c| c.is_alphanumeric() || (c == '_'));
            Ok(())
        })
    }

    // Reference: https://docs.python.org/3/reference/lexical_analysis.html#keywords
    fn scan_python_keyword(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            let (r, s) = scanner.scan_python_identifier()?;
            if s.is_python_keyword() {
                Ok(())
            } else {
                Err((r, s))
            }
        })
    }

    // Reference: https://docs.python.org/3/reference/lexical_analysis.html#soft-keywords
    fn scan_python_soft_keyword(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            let (r, s) = scanner.scan_python_identifier()?;
            if s.is_python_soft_keyword() {
                Ok(())
            } else {
                Err((r, s))
            }
        })
    }

    // Reference: https://docs.python.org/3/reference/lexical_analysis.html#operators
    // Reference: https://docs.python.org/3/reference/lexical_analysis.html#delimiters
    fn scan_python_operator(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            let (r, c) = scanner.next()?;
            match c {
                '=' => {
                    _ = scanner.accept_char('=');
                }
                '/' => {
                    _ = scanner.accept_char('/');
                    _ = scanner.accept_char('=');
                }
                '-' => {
                    _ = scanner.accept_char_any(&['=', '>']);
                }
                '+' | '%' | '&' | '|' | '^' => {
                    _ = scanner.accept_char('=');
                }
                '*' => {
                    _ = scanner.accept_char('*');
                    _ = scanner.accept_char('=');
                }
                '<' => {
                    _ = scanner.accept_char('<');
                    _ = scanner.accept_char('=');
                }
                '>' => {
                    _ = scanner.accept_char('>');
                    _ = scanner.accept_char('=');
                }
                '@' => {
                    _ = scanner.accept_char('=');
                }
                ':' => {
                    _ = scanner.accept_char('=');
                }
                '!' => {
                    scanner.accept_char('=')?;
                }
                ',' | '.' | ';' | '~' => {}
                _ => return Err(scanner.ranged_text(r)),
            }
            Ok(())
        })
    }

    // Reference: https://docs.python.org/3/reference/lexical_analysis.html#delimiters
    fn scan_python_delimiter(&mut self) -> ScannerResult<'text, &'text str> {
        let (r, c) = self.peek()?;
        let ret = self.ranged_text(r);
        match c {
            '(' | ')' | '[' | ']' | '{' | '}' => {
                self.cursor = ret.0.end;
                Ok(ret)
            }
            _ => Err(ret),
        }
    }

    // Reference: https://docs.python.org/3/reference/lexical_analysis.html#integer-literals
    fn scan_python_int_dec(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_if_ext(char::is_ascii_digit)?;
            scanner.skip_while(|c| c.is_ascii_digit() || (c == '_'));
            Ok(())
        })
    }

    // Reference: https://docs.python.org/3/reference/lexical_analysis.html#integer-literals
    fn scan_python_int_hex(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('0')?;
            scanner.accept_char_any(&['x', 'X'])?;

            scanner.skip_while_char('_');
            scanner.accept_if_ext(char::is_ascii_hexdigit)?;

            scanner.skip_while(|c| c.is_ascii_hexdigit() || (c == '_'));

            Ok(())
        })
    }

    // Reference: https://docs.python.org/3/reference/lexical_analysis.html#integer-literals
    fn scan_python_int_oct(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('0')?;
            scanner.accept_char_any(&['o', 'O'])?;

            scanner.skip_while_char('_');
            scanner.accept_if(CharExt::is_ascii_octdigit)?;

            scanner.skip_while(|c| CharExt::is_ascii_octdigit(c) || (c == '_'));

            Ok(())
        })
    }

    // Reference: https://docs.python.org/3/reference/lexical_analysis.html#integer-literals
    fn scan_python_int_bin(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('0')?;
            scanner.accept_char_any(&['b', 'B'])?;

            scanner.skip_while_char('_');
            scanner.accept_if(CharExt::is_ascii_bindigit)?;

            scanner.skip_while(|c| c.is_ascii_bindigit() || (c == '_'));

            Ok(())
        })
    }

    // Reference: https://docs.python.org/3/reference/lexical_analysis.html#floating-point-literals
    fn scan_python_float(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            let mut int_range = None;

            if scanner.accept_char('.').is_ok() {
                scanner.scan_python_int_dec()?;
            } else {
                int_range = Some(scanner.scan_python_int_dec()?.0);

                if scanner.accept_char('.').is_ok() {
                    int_range = None;
                    _ = scanner.scan_python_int_dec();
                }
            }

            if scanner.accept_char_any(&['e', 'E']).is_ok() {
                _ = scanner.accept_char_any(&['+', '-']);

                scanner.skip_while_char('_');
                scanner.accept_if_ext(char::is_ascii_digit)?;
                scanner.skip_while(|c| c.is_ascii_digit() || (c == '_'));
            } else if let Some(r) = int_range {
                return Err(scanner.ranged_text(r));
            }

            Ok(())
        })
    }

    // Reference: https://docs.python.org/3/reference/lexical_analysis.html#string-and-bytes-literals
    #[inline]
    fn scan_python_string(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_python_long_string()
            .or_else(|_| self.scan_python_short_string())
    }

    // Reference: https://docs.python.org/3/reference/lexical_analysis.html#string-and-bytes-literals
    fn scan_python_short_string(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scan_python_string_prefix(scanner)?;
            scan_python_short_string(scanner)?;
            Ok(())
        })
    }

    // Reference: https://docs.python.org/3/reference/lexical_analysis.html#string-and-bytes-literals
    fn scan_python_long_string(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scan_python_string_prefix(scanner)?;
            scan_python_long_string(scanner)?;
            Ok(())
        })
    }

    // Reference: https://docs.python.org/3/reference/lexical_analysis.html#string-and-bytes-literals
    #[inline]
    fn scan_python_bytes(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_python_long_bytes()
            .or_else(|_| self.scan_python_short_bytes())
    }

    // Reference: https://docs.python.org/3/reference/lexical_analysis.html#string-and-bytes-literals
    fn scan_python_short_bytes(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scan_python_bytes_prefix(scanner)?;
            // Note: Does not validate the bytes string contents
            scan_python_short_string(scanner)?;
            Ok(())
        })
    }

    // Reference: https://docs.python.org/3/reference/lexical_analysis.html#string-and-bytes-literals
    fn scan_python_long_bytes(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scan_python_bytes_prefix(scanner)?;
            // Note: Does not validate the bytes string contents
            scan_python_long_string(scanner)?;
            Ok(())
        })
    }
}

// Reference: https://docs.python.org/3/reference/lexical_analysis.html#string-and-bytes-literals
#[inline]
fn scan_python_string_prefix<'text>(scanner: &mut Scanner<'text>) -> ScanResult<'text> {
    let c = match scanner.accept_char_any(&['r', 'R', 'f', 'F', 'u', 'U']) {
        Ok((_r, c)) => c,
        Err(_) => return Ok(()),
    };

    match c {
        'f' | 'F' => {
            _ = scanner.accept_char_any(&['r', 'R']);
        }
        'r' | 'R' => {
            _ = scanner.accept_char_any(&['f', 'F']);
        }
        'u' | 'U' => {}
        _ => unreachable!(),
    }

    Ok(())
}

// Reference: https://docs.python.org/3/reference/lexical_analysis.html#string-and-bytes-literals
#[inline]
fn scan_python_bytes_prefix<'text>(scanner: &mut Scanner<'text>) -> ScanResult<'text> {
    let c = match scanner.accept_char_any(&['b', 'B', 'r', 'R']) {
        Ok((_r, c)) => c,
        Err(_) => return Ok(()),
    };

    match c {
        'b' | 'B' => {
            _ = scanner.accept_char_any(&['r', 'R']);
        }
        'r' | 'R' => {
            scanner.accept_char_any(&['b', 'B'])?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

// Reference: https://docs.python.org/3/reference/lexical_analysis.html#string-and-bytes-literals
#[inline]
fn scan_python_short_string<'text>(scanner: &mut Scanner<'text>) -> ScanResult<'text> {
    let (_r, quote) = scanner.accept_char_any(&['"', '\''])?;

    loop {
        scanner.skip_until_char_any(&[quote, '\\', '\n']);
        match scanner.peek() {
            Ok((_r, c)) if c == quote => {
                _ = scanner.next();
                break;
            }
            Ok((_r, '\\')) => {
                _ = scanner.next();
                // Skip the next character as it is escaped
                // Note: Technically any character is not valid
                _ = scanner.next();
            }
            Ok((_r, '\n')) => break,
            Ok(_) => unreachable!(),
            Err(_) => break,
        }
    }

    Ok(())
}

// Reference: https://docs.python.org/3/reference/lexical_analysis.html#string-and-bytes-literals
#[inline]
fn scan_python_long_string<'text>(scanner: &mut Scanner<'text>) -> ScanResult<'text> {
    let (_r, quote) = scanner.accept_char_any(&['"', '\''])?;
    scanner.accept_char(quote)?;
    scanner.accept_char(quote)?;

    'scan: loop {
        scanner.skip_until_char_any(&[quote, '\\']);
        match scanner.peek() {
            Ok((_r, c)) if c == quote => {
                _ = scanner.next();

                for _ in 0..2 {
                    if scanner.accept_char(quote).is_err() {
                        continue 'scan;
                    }
                }

                break;
            }
            Ok((_r, '\\')) => {
                _ = scanner.next();
                // Skip the next character as it is escaped
                // Note: Technically any character is not valid
                _ = scanner.next();
            }
            Ok(_) => unreachable!(),
            Err(_) => break,
        }
    }

    Ok(())
}

/// [`str`] extension for checking if a `&str` is e.g. a Python keyword.
pub trait PythonStrExt {
    fn is_python_keyword(&self) -> bool;
    fn is_python_soft_keyword(&self) -> bool;
    fn is_python_operator(&self) -> bool;
    fn is_python_delimiter(&self) -> bool;
}

impl PythonStrExt for str {
    #[inline]
    fn is_python_keyword(&self) -> bool {
        PYTHON_KEYWORDS.contains(&self)
    }

    #[inline]
    fn is_python_soft_keyword(&self) -> bool {
        PYTHON_SOFT_KEYWORDS.contains(&self)
    }

    #[inline]
    fn is_python_operator(&self) -> bool {
        PYTHON_OPERATORS.contains(&self)
    }

    #[inline]
    fn is_python_delimiter(&self) -> bool {
        PYTHON_DELIMITERS.contains(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_line_comment() {
        let cases = [
            // text, expected, remaining text
            ("#", Ok((0..1, "#")), ""),
            ("#\n", Ok((0..1, "#")), "\n"),
            ("#\r\n", Ok((0..1, "#")), "\r\n"),
            //
            ("# Line Comment", Ok((0..14, "# Line Comment")), ""),
            ("# Line Comment\n", Ok((0..14, "# Line Comment")), "\n"),
            ("# Line Comment\r\n", Ok((0..14, "# Line Comment")), "\r\n"),
            //
            ("", Err((0..0, "")), ""),
            (" #", Err((0..0, "")), " #"),
            (" #\n", Err((0..0, "")), " #\n"),
            (" #\r\n", Err((0..0, "")), " #\r\n"),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);

            let actual = scanner.scan_python_line_comment();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_python_explicit_line_joiner() {
        let cases = [
            // text, expected, remaining text
            ("\\", Ok((0..1, "\\")), ""),
            ("\\\n", Ok((0..1, "\\")), "\n"),
            ("\\\r\n", Ok((0..1, "\\")), "\r\n"),
            ("\\\r", Ok((0..1, "\\")), "\r"),
            //
            ("\\Foo", Err((0..1, "\\")), "\\Foo"),
            ("\\\nFoo", Ok((0..1, "\\")), "\nFoo"),
            ("\\\r\nFoo", Ok((0..1, "\\")), "\r\nFoo"),
            //
            ("\\ Foo", Err((0..1, "\\")), "\\ Foo"),
            ("\\\n Foo", Ok((0..1, "\\")), "\n Foo"),
            ("\\\r\n Foo", Ok((0..1, "\\")), "\r\n Foo"),
            //
            ("\\\rFoo", Err((0..1, "\\")), "\\\rFoo"),
            ("\\ Foo", Err((0..1, "\\")), "\\ Foo"),
            ("\\ \rFoo", Err((0..1, "\\")), "\\ \rFoo"),
            //
            ("\\\\", Err((0..1, "\\")), "\\\\"),
            ("\\\\\n", Err((0..1, "\\")), "\\\\\n"),
            ("\\\\\r\n", Err((0..1, "\\")), "\\\\\r\n"),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);

            let actual = scanner.scan_python_explicit_line_joiner();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_python_identifier() {
        let cases = [
            // text, expected, remaining text
            ("x", Ok((0..1, "x")), ""),
            ("_", Ok((0..1, "_")), ""),
            ("x_", Ok((0..2, "x_")), ""),
            ("xyz", Ok((0..3, "xyz")), ""),
            ("x_y_z", Ok((0..5, "x_y_z")), ""),
            ("_x_y_z_", Ok((0..7, "_x_y_z_")), ""),
            //
            ("x1", Ok((0..2, "x1")), ""),
            ("_1", Ok((0..2, "_1")), ""),
            //
            ("x ", Ok((0..1, "x")), " "),
            ("x\t", Ok((0..1, "x")), "\t"),
            ("x\n", Ok((0..1, "x")), "\n"),
            ("x\r\n", Ok((0..1, "x")), "\r\n"),
            //
            ("x-", Ok((0..1, "x")), "-"),
            ("x+", Ok((0..1, "x")), "+"),
            ("x()", Ok((0..1, "x")), "()"),
            //
            ("_2-", Ok((0..2, "_2")), "-"),
            ("_-2", Ok((0..1, "_")), "-2"),
            //
            ("", Err((0..0, "")), ""),
            (" x", Err((0..0, "")), " x"),
            ("\tx", Err((0..0, "")), "\tx"),
            ("\nx", Err((0..0, "")), "\nx"),
            //
            ("1x", Err((0..0, "")), "1x"),
            ("-x", Err((0..0, "")), "-x"),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);

            let actual = scanner.scan_python_identifier();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_python_keyword() {
        for &expected in PYTHON_KEYWORDS {
            let mut scanner = Scanner::new(expected);

            let actual = scanner.scan_python_keyword().map(|(_r, kw)| kw);
            assert_eq!(actual, Ok(expected));
        }
    }

    #[test]
    fn test_python_soft_keyword() {
        for &expected in PYTHON_SOFT_KEYWORDS {
            let mut scanner = Scanner::new(expected);

            let actual = scanner.scan_python_soft_keyword().map(|(_r, kw)| kw);
            assert_eq!(actual, Ok(expected));
        }
    }

    #[test]
    fn test_python_operator() {
        for &expected in PYTHON_OPERATORS {
            let mut scanner = Scanner::new(expected);

            let actual = scanner.scan_python_operator().map(|(_r, kw)| kw);
            assert_eq!(actual, Ok(expected));
        }
    }

    #[test]
    fn test_python_delimiter() {
        for &expected in PYTHON_DELIMITERS {
            let mut scanner = Scanner::new(expected);

            let actual = scanner.scan_python_delimiter().map(|(_r, kw)| kw);
            assert_eq!(actual, Ok(expected));
        }
    }

    #[test]
    fn test_python_int_dec() {
        let cases = [
            // text, expected, remaining text
            ("0", Ok((0..1, "0")), ""),
            ("1", Ok((0..1, "1")), ""),
            ("123", Ok((0..3, "123")), ""),
            ("1234567890", Ok((0..10, "1234567890")), ""),
            //
            ("0+", Ok((0..1, "0")), "+"),
            //
            ("1_2", Ok((0..3, "1_2")), ""),
            // FIXME: ("1_2_", Ok((0..3, "1_2")), ""),
            ("_1_2", Err((0..0, "")), "_1_2"),
            //
            // FIXME: ("0123", Err((0..1, "0")), "0123"),
            // FIXME: ("12__34", Err((0..3, "12_")), "12__34"),
            //
            ("-0", Err((0..0, "")), "-0"),
            ("-123", Err((0..0, "")), "-123"),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);

            let actual = scanner.scan_python_int_dec();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_python_int_hex() {
        let cases = [
            // text, expected, remaining text
            ("0x0", Ok((0..3, "0x0")), ""),
            ("0xF", Ok((0..3, "0xF")), ""),
            ("0xf", Ok((0..3, "0xf")), ""),
            ("0xFF", Ok((0..4, "0xFF")), ""),
            //
            ("0X0", Ok((0..3, "0X0")), ""),
            ("0XF", Ok((0..3, "0XF")), ""),
            ("0Xf", Ok((0..3, "0Xf")), ""),
            ("0XFF", Ok((0..4, "0XFF")), ""),
            //
            ("0xFFF", Ok((0..5, "0xFFF")), ""),
            ("0xFFFFFF", Ok((0..8, "0xFFFFFF")), ""),
            ("0xFFFFFFFFFFFF", Ok((0..14, "0xFFFFFFFFFFFF")), ""),
            ("0x0123456789ABCDEF", Ok((0..18, "0x0123456789ABCDEF")), ""),
            ("0x0123456789abcdef", Ok((0..18, "0x0123456789abcdef")), ""),
            //
            ("0xFF+", Ok((0..4, "0xFF")), "+"),
            //
            ("0xF_F", Ok((0..5, "0xF_F")), ""),
            ("0x_FF", Ok((0..5, "0x_FF")), ""),
            ("0x_F_F", Ok((0..6, "0x_F_F")), ""),
            ("0x_", Err((0..3, "0x_")), "0x_"),
            // FIXME: ("0xF__F", Err((0..4, "0xF_")), "0xF__F"),
            //
            ("0", Err((0..1, "0")), "0"),
            ("0x", Err((0..2, "0x")), "0x"),
            //
            ("1x", Err((0..0, "")), "1x"),
            ("1xF", Err((0..0, "")), "1xF"),
            ("1xFF", Err((0..0, "")), "1xFF"),
            //
            ("-0xFF", Err((0..0, "")), "-0xFF"),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);

            let actual = scanner.scan_python_int_hex();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_python_int_oct() {
        let cases = [
            // text, expected, remaining text
            ("0o0", Ok((0..3, "0o0")), ""),
            ("0o7", Ok((0..3, "0o7")), ""),
            ("0o00", Ok((0..4, "0o00")), ""),
            ("0o77", Ok((0..4, "0o77")), ""),
            ("0o1234567", Ok((0..9, "0o1234567")), ""),
            //
            ("0O0", Ok((0..3, "0O0")), ""),
            ("0O7", Ok((0..3, "0O7")), ""),
            ("0O00", Ok((0..4, "0O00")), ""),
            ("0O77", Ok((0..4, "0O77")), ""),
            ("0O1234567", Ok((0..9, "0O1234567")), ""),
            //
            ("0o77+", Ok((0..4, "0o77")), "+"),
            //
            ("0o7_7", Ok((0..5, "0o7_7")), ""),
            ("0o_77", Ok((0..5, "0o_77")), ""),
            ("0o_7_7", Ok((0..6, "0o_7_7")), ""),
            ("0o_", Err((0..3, "0o_")), "0o_"),
            // FIXME: ("0o7__7", Err((0..4, "0o7_")), "0o7__7"),
            //
            ("0", Err((0..1, "0")), "0"),
            ("0o", Err((0..2, "0o")), "0o"),
            //
            ("1", Err((0..0, "")), "1"),
            ("1o", Err((0..0, "")), "1o"),
            ("1o77", Err((0..0, "")), "1o77"),
            ("1o2345670", Err((0..0, "")), "1o2345670"),
            //
            ("-0o77", Err((0..0, "")), "-0o77"),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);

            let actual = scanner.scan_python_int_oct();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_python_int_bin() {
        let cases = [
            // text, expected, remaining text
            ("0b0", Ok((0..3, "0b0")), ""),
            ("0b1", Ok((0..3, "0b1")), ""),
            ("0b2", Err((0..2, "0b")), "0b2"),
            ("0B0", Ok((0..3, "0B0")), ""),
            ("0B1", Ok((0..3, "0B1")), ""),
            ("0B2", Err((0..2, "0B")), "0B2"),
            //
            ("0b0000", Ok((0..6, "0b0000")), ""),
            ("0b1111", Ok((0..6, "0b1111")), ""),
            ("0b0011", Ok((0..6, "0b0011")), ""),
            ("0b1100", Ok((0..6, "0b1100")), ""),
            //
            ("1b0", Err((0..0, "")), "1b0"),
            ("1b1", Err((0..0, "")), "1b1"),
            ("1B0", Err((0..0, "")), "1B0"),
            ("1B1", Err((0..0, "")), "1B1"),
            //
            ("0b0+", Ok((0..3, "0b0")), "+"),
            ("0b1+", Ok((0..3, "0b1")), "+"),
            //
            ("-0b0", Err((0..0, "")), "-0b0"),
            ("-0b1", Err((0..0, "")), "-0b1"),
            (" 0b0", Err((0..0, "")), " 0b0"),
            (" 0b1", Err((0..0, "")), " 0b1"),
            //
            ("0b1_1", Ok((0..5, "0b1_1")), ""),
            ("0b_11", Ok((0..5, "0b_11")), ""),
            ("0b_1_1", Ok((0..6, "0b_1_1")), ""),
            ("0b_", Err((0..3, "0b_")), "0b_"),
            // FIXME: ("0b1__1", Err((0..4, "0b1_")), "0b1__1"),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);

            let actual = scanner.scan_python_int_bin();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_python_float() {
        let cases = [
            // text, expected, remaining text
            ("1.", Ok((0..2, "1.")), ""),
            (".2", Ok((0..2, ".2")), ""),
            ("1.2", Ok((0..3, "1.2")), ""),
            //
            ("1.2E3", Ok((0..5, "1.2E3")), ""),
            ("1.2E+3", Ok((0..6, "1.2E+3")), ""),
            ("1.2E-3", Ok((0..6, "1.2E-3")), ""),
            ("1.2e3", Ok((0..5, "1.2e3")), ""),
            ("1.2e+3", Ok((0..6, "1.2e+3")), ""),
            ("1.2e-3", Ok((0..6, "1.2e-3")), ""),
            //
            ("12345.", Ok((0..6, "12345.")), ""),
            (".12345", Ok((0..6, ".12345")), ""),
            ("12345.12345", Ok((0..11, "12345.12345")), ""),
            ("12345.12345E+12345", Ok((0..18, "12345.12345E+12345")), ""),
            //
            ("0e0", Ok((0..3, "0e0")), ""),
            (".001", Ok((0..4, ".001")), ""),
            ("1e100", Ok((0..5, "1e100")), ""),
            ("3.14_15_93", Ok((0..10, "3.14_15_93")), ""),
            //
            ("1. ", Ok((0..2, "1.")), " "),
            (".2 ", Ok((0..2, ".2")), " "),
            ("1.2 ", Ok((0..3, "1.2")), " "),
            ("1.2\n", Ok((0..3, "1.2")), "\n"),
            //
            ("1.+", Ok((0..2, "1.")), "+"),
            (".2+", Ok((0..2, ".2")), "+"),
            ("1.2+", Ok((0..3, "1.2")), "+"),
            //
            (" 1.", Err((0..0, "")), " 1."),
            (" .2", Err((0..0, "")), " .2"),
            (" 1.2", Err((0..0, "")), " 1.2"),
            //
            ("0", Err((0..1, "0")), "0"),
            ("100", Err((0..3, "100")), "100"),
            //
            ("-1", Err((0..0, "")), "-1"),
            ("-1.", Err((0..0, "")), "-1."),
            ("-.2", Err((0..0, "")), "-.2"),
            ("-1.2", Err((0..0, "")), "-1.2"),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);

            let actual = scanner.scan_python_float();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_python_short_string_double_quote() {
        let cases = [
            // text, expected, remaining text
            ("\"\"", Ok((0..2, "\"\"")), ""),
            ("\" \"", Ok((0..3, "\" \"")), ""),
            ("\"Foo Bar\"", Ok((0..9, "\"Foo Bar\"")), ""),
            //
            ("\"Foo \n Bar\"", Ok((0..5, "\"Foo ")), "\n Bar\""),
            ("\"Foo \\n Bar\"", Ok((0..12, "\"Foo \\n Bar\"")), ""),
            //
            ("\"Foo \\\" Bar\"", Ok((0..12, "\"Foo \\\" Bar\"")), ""),
            ("\"Foo \\\n Bar\"", Ok((0..12, "\"Foo \\\n Bar\"")), ""),
            //
            ("\"\" ", Ok((0..2, "\"\"")), " "),
            ("\"\"\t", Ok((0..2, "\"\"")), "\t"),
            ("\"\"\n", Ok((0..2, "\"\"")), "\n"),
            ("\"\"\r\n", Ok((0..2, "\"\"")), "\r\n"),
            //
            ("", Err((0..0, "")), ""),
            //
            (" \"\"", Err((0..0, "")), " \"\""),
            ("\t\"\"", Err((0..0, "")), "\t\"\""),
            ("\n\"\"", Err((0..0, "")), "\n\"\""),
            //
            ("\"", Ok((0..1, "\"")), ""),
            ("\" ", Ok((0..2, "\" ")), ""),
            ("\"\n", Ok((0..1, "\"")), "\n"),
            ("\"Foo\n", Ok((0..4, "\"Foo")), "\n"),
            ("\"Foo\nBar\"", Ok((0..4, "\"Foo")), "\nBar\""),
            //
            ("r\"\"", Ok((0..3, "r\"\"")), ""),
            ("u\"\"", Ok((0..3, "u\"\"")), ""),
            ("R\"\"", Ok((0..3, "R\"\"")), ""),
            ("U\"\"", Ok((0..3, "U\"\"")), ""),
            ("f\"\"", Ok((0..3, "f\"\"")), ""),
            ("F\"\"", Ok((0..3, "F\"\"")), ""),
            ("fr\"\"", Ok((0..4, "fr\"\"")), ""),
            ("Fr\"\"", Ok((0..4, "Fr\"\"")), ""),
            ("fR\"\"", Ok((0..4, "fR\"\"")), ""),
            ("FR\"\"", Ok((0..4, "FR\"\"")), ""),
            ("rf\"\"", Ok((0..4, "rf\"\"")), ""),
            ("rF\"\"", Ok((0..4, "rF\"\"")), ""),
            ("Rf\"\"", Ok((0..4, "Rf\"\"")), ""),
            ("RF\"\"", Ok((0..4, "RF\"\"")), ""),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);

            let actual = scanner.scan_python_short_string();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_python_short_string_single_quote() {
        let cases = [
            // text, expected, remaining text
            ("''", Ok((0..2, "''")), ""),
            ("' '", Ok((0..3, "' '")), ""),
            ("'Foo Bar'", Ok((0..9, "'Foo Bar'")), ""),
            //
            ("'Foo \n Bar'", Ok((0..5, "'Foo ")), "\n Bar'"),
            ("'Foo \\n Bar'", Ok((0..12, "'Foo \\n Bar'")), ""),
            //
            ("'Foo \\' Bar'", Ok((0..12, "'Foo \\' Bar'")), ""),
            ("'Foo \\\n Bar'", Ok((0..12, "'Foo \\\n Bar'")), ""),
            //
            ("'' ", Ok((0..2, "''")), " "),
            ("''\t", Ok((0..2, "''")), "\t"),
            ("''\n", Ok((0..2, "''")), "\n"),
            ("''\r\n", Ok((0..2, "''")), "\r\n"),
            //
            ("", Err((0..0, "")), ""),
            //
            (" ''", Err((0..0, "")), " ''"),
            ("\t''", Err((0..0, "")), "\t''"),
            ("\n''", Err((0..0, "")), "\n''"),
            //
            ("'", Ok((0..1, "'")), ""),
            ("' ", Ok((0..2, "' ")), ""),
            ("'\n", Ok((0..1, "'")), "\n"),
            ("'Foo\n", Ok((0..4, "'Foo")), "\n"),
            ("'Foo\nBar'", Ok((0..4, "'Foo")), "\nBar'"),
            //
            ("r''", Ok((0..3, "r''")), ""),
            ("u''", Ok((0..3, "u''")), ""),
            ("R''", Ok((0..3, "R''")), ""),
            ("U''", Ok((0..3, "U''")), ""),
            ("f''", Ok((0..3, "f''")), ""),
            ("F''", Ok((0..3, "F''")), ""),
            ("fr''", Ok((0..4, "fr''")), ""),
            ("Fr''", Ok((0..4, "Fr''")), ""),
            ("fR''", Ok((0..4, "fR''")), ""),
            ("FR''", Ok((0..4, "FR''")), ""),
            ("rf''", Ok((0..4, "rf''")), ""),
            ("rF''", Ok((0..4, "rF''")), ""),
            ("Rf''", Ok((0..4, "Rf''")), ""),
            ("RF''", Ok((0..4, "RF''")), ""),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);

            let actual = scanner.scan_python_short_string();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_python_long_string_double_quote() {
        #[rustfmt::skip]
        let cases = [
            // text, expected, remaining text
            ("\"\"\"\"\"\"", Ok((0..6, "\"\"\"\"\"\"")), ""),
            ("\"\"\" \"\"\"", Ok((0..7, "\"\"\" \"\"\"")), ""),
            ("\"\"\"Foo Bar\"\"\"", Ok((0..13, "\"\"\"Foo Bar\"\"\"")), ""),
            //
            ("\"\"\"Foo\nBar\"\"\"", Ok((0..13, "\"\"\"Foo\nBar\"\"\"")), ""),
            //
            ("\"\"\" \" \"\" \"\"\"", Ok((0..12, "\"\"\" \" \"\" \"\"\"")), ""),
            ("\"\"\"\\\"\"\"Foo\"\"\"", Ok((0..13, "\"\"\"\\\"\"\"Foo\"\"\"")), ""),
            ("\"\"\"\"Foo\"\"\"\"", Ok((0..10, "\"\"\"\"Foo\"\"\"")), "\""),
            //
            ("\"\"\"Foo'''\"\"\"\"", Ok((0..12, "\"\"\"Foo'''\"\"\"")), "\""),
            //
            ("\"\"\"Foo\"\"", Ok((0..8, "\"\"\"Foo\"\"")), ""),
            ("\"\"\"Foo\n\"\"", Ok((0..9, "\"\"\"Foo\n\"\"")), ""),
            //
            ("r\"\"\"\"\"\"", Ok((0..7, "r\"\"\"\"\"\"")), ""),
            ("u\"\"\"\"\"\"", Ok((0..7, "u\"\"\"\"\"\"")), ""),
            ("R\"\"\"\"\"\"", Ok((0..7, "R\"\"\"\"\"\"")), ""),
            ("U\"\"\"\"\"\"", Ok((0..7, "U\"\"\"\"\"\"")), ""),
            ("f\"\"\"\"\"\"", Ok((0..7, "f\"\"\"\"\"\"")), ""),
            ("F\"\"\"\"\"\"", Ok((0..7, "F\"\"\"\"\"\"")), ""),
            ("fr\"\"\"\"\"\"", Ok((0..8, "fr\"\"\"\"\"\"")), ""),
            ("Fr\"\"\"\"\"\"", Ok((0..8, "Fr\"\"\"\"\"\"")), ""),
            ("fR\"\"\"\"\"\"", Ok((0..8, "fR\"\"\"\"\"\"")), ""),
            ("FR\"\"\"\"\"\"", Ok((0..8, "FR\"\"\"\"\"\"")), ""),
            ("rf\"\"\"\"\"\"", Ok((0..8, "rf\"\"\"\"\"\"")), ""),
            ("rF\"\"\"\"\"\"", Ok((0..8, "rF\"\"\"\"\"\"")), ""),
            ("Rf\"\"\"\"\"\"", Ok((0..8, "Rf\"\"\"\"\"\"")), ""),
            ("RF\"\"\"\"\"\"", Ok((0..8, "RF\"\"\"\"\"\"")), ""),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);

            let actual = scanner.scan_python_long_string();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_python_long_string_single_quote() {
        let cases = [
            // text, expected, remaining text
            ("''''''", Ok((0..6, "''''''")), ""),
            ("''' '''", Ok((0..7, "''' '''")), ""),
            ("'''Foo Bar'''", Ok((0..13, "'''Foo Bar'''")), ""),
            //
            ("'''Foo\nBar'''", Ok((0..13, "'''Foo\nBar'''")), ""),
            //
            ("''' ' '' '''", Ok((0..12, "''' ' '' '''")), ""),
            ("'''\\'''Foo'''", Ok((0..13, "'''\\'''Foo'''")), ""),
            ("''''Foo''''", Ok((0..10, "''''Foo'''")), "'"),
            //
            ("'''Foo\"\"\"''''", Ok((0..12, "'''Foo\"\"\"'''")), "'"),
            //
            ("'''Foo''", Ok((0..8, "'''Foo''")), ""),
            ("'''Foo\n''", Ok((0..9, "'''Foo\n''")), ""),
            //
            ("r''''''", Ok((0..7, "r''''''")), ""),
            ("u''''''", Ok((0..7, "u''''''")), ""),
            ("R''''''", Ok((0..7, "R''''''")), ""),
            ("U''''''", Ok((0..7, "U''''''")), ""),
            ("f''''''", Ok((0..7, "f''''''")), ""),
            ("F''''''", Ok((0..7, "F''''''")), ""),
            ("fr''''''", Ok((0..8, "fr''''''")), ""),
            ("Fr''''''", Ok((0..8, "Fr''''''")), ""),
            ("fR''''''", Ok((0..8, "fR''''''")), ""),
            ("FR''''''", Ok((0..8, "FR''''''")), ""),
            ("rf''''''", Ok((0..8, "rf''''''")), ""),
            ("rF''''''", Ok((0..8, "rF''''''")), ""),
            ("Rf''''''", Ok((0..8, "Rf''''''")), ""),
            ("RF''''''", Ok((0..8, "RF''''''")), ""),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);

            let actual = scanner.scan_python_long_string();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }
}
