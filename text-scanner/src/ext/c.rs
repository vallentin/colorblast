use crate::{CharExt, Scanner, ScannerResult};

pub trait CScannerExt<'text> {
    fn scan_c_line_comment(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_c_block_comment(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_c_identifier(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_c_int_dec(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_c_int_hex(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_c_int_oct(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_c_float(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_c_char(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_c_string(&mut self) -> ScannerResult<'text, &'text str>;
}

impl<'text> CScannerExt<'text> for Scanner<'text> {
    // Reference: https://learn.microsoft.com/en-us/cpp/c-language/c-comments?view=msvc-170
    fn scan_c_line_comment(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_str("//")?;
            scanner.skip_until_char_any(&['\n', '\r']);
            Ok(())
        })
    }

    // Reference: https://learn.microsoft.com/en-us/cpp/c-language/c-comments?view=msvc-170
    fn scan_c_block_comment(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_str("/*")?;

            loop {
                scanner.skip_until_char('*');

                match scanner.next() {
                    Ok((_r, '*')) => {
                        if let Ok((_r, '/')) = scanner.next() {
                            break;
                        }
                    }
                    Ok((_r, _c)) => {}
                    Err(_) => break,
                }
            }
            Ok(())
        })
    }

    // Reference: https://learn.microsoft.com/en-us/cpp/c-language/c-identifiers?view=msvc-170#syntax
    fn scan_c_identifier(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_if(|c| c.is_alphabetic() || (c == '_'))?;
            scanner.skip_while(|c| c.is_alphanumeric() || (c == '_'));
            Ok(())
        })
    }

    fn scan_c_int_dec(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_if_ext(char::is_ascii_digit)?;
            scanner.skip_while_ext(char::is_ascii_digit);
            Ok(())
        })
    }

    fn scan_c_int_hex(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('0')?;
            scanner.accept_char_any(&['x', 'X'])?;

            scanner.accept_if_ext(char::is_ascii_hexdigit)?;
            scanner.skip_while_ext(char::is_ascii_hexdigit);

            Ok(())
        })
    }

    fn scan_c_int_oct(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('0')?;

            scanner.accept_if(CharExt::is_ascii_octdigit)?;
            scanner.skip_while(CharExt::is_ascii_octdigit);

            Ok(())
        })
    }

    fn scan_c_float(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            if scanner.accept_char('.').is_ok() {
                scanner.scan_c_int_dec()?;
            } else {
                scanner.scan_c_int_dec()?;
                scanner.accept_char('.')?;
                _ = scanner.scan_c_int_dec();
            }

            if scanner.accept_char_any(&['e', 'E']).is_ok() {
                _ = scanner.accept_char_any(&['+', '-']);

                scanner.accept_if_ext(char::is_ascii_digit)?;
                scanner.skip_while_ext(char::is_ascii_digit);
            }

            Ok(())
        })
    }

    // Reference: https://learn.microsoft.com/en-us/cpp/c-language/c-character-constants?view=msvc-170#syntax
    fn scan_c_char(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('\'')?;

            let (_r, c) = scanner.next()?;
            if c == '\\' {
                // Skip the next character as it is escaped
                // Note: Technically any character is not valid
                let (_r, c) = scanner.next()?;

                if CharExt::is_ascii_octdigit(c) {
                    _ = scanner.accept_if(CharExt::is_ascii_octdigit);
                    _ = scanner.accept_if(CharExt::is_ascii_octdigit);
                } else if c == 'x' {
                    scanner.accept_if_ext(char::is_ascii_hexdigit)?;
                    _ = scanner.accept_if_ext(char::is_ascii_hexdigit);
                }
            }

            scanner.accept_char('\'')?;
            Ok(())
        })
    }

    // Reference: https://learn.microsoft.com/en-us/cpp/c-language/c-string-literals?view=msvc-170#syntax
    fn scan_c_string(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('"')?;

            loop {
                scanner.skip_until_char_any(&['"', '\\', '\n']);
                match scanner.peek() {
                    Ok((_r, '"')) => {
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
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_line_comment() {
        #[rustfmt::skip]
        let cases = [
            // text, expected, remaining text
            ("//", Ok((0..2, "//")), ""),
            ("//\n", Ok((0..2, "//")), "\n"),
            ("//\r\n", Ok((0..2, "//")), "\r\n"),
            //
            ("// Line Comment", Ok((0..15, "// Line Comment")), ""),
            ("// Line Comment\n", Ok((0..15, "// Line Comment")), "\n"),
            ("// Line Comment\r\n", Ok((0..15, "// Line Comment")), "\r\n"),
            //
            ("", Err((0..0, "")), ""),
            ("/", Err((0..1, "/")), "/"),
            (" //", Err((0..0, "")), " //"),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);

            let actual = scanner.scan_c_line_comment();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_c_block_comment() {
        #[rustfmt::skip]
        let cases = [
            // text, expected, remaining text
            ("/**/", Ok((0..4, "/**/")), ""),
            ("/**/\n", Ok((0..4, "/**/")), "\n"),
            //
            ("/*\nBlock\nComment\n*/\n", Ok((0..19, "/*\nBlock\nComment\n*/")), "\n"),
            ("/*\r\nBlock\r\nComment\r\n*/\r\n", Ok((0..22, "/*\r\nBlock\r\nComment\r\n*/")), "\r\n"),
            //
            ("", Err((0..0, "")), ""),
            ("/ **/", Err((0..1, "/")), "/ **/"),
            (" /**/", Err((0..0, "")), " /**/"),
            //
            ("/*", Ok((0..2, "/*")), ""),
            ("/* ", Ok((0..3, "/* ")), ""),
            ("/* * /", Ok((0..6, "/* * /")), ""),
            ("/* * /\n", Ok((0..7, "/* * /\n")), ""),
            ("/* Unterminated Block Comment", Ok((0..29, "/* Unterminated Block Comment")), ""),
            ("/*\nUnterminated\nBlock\nComment\n", Ok((0..30, "/*\nUnterminated\nBlock\nComment\n")), ""),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);

            let actual = scanner.scan_c_block_comment();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_c_identifier() {
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

            let actual = scanner.scan_c_identifier();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_c_int_dec() {
        let cases = [
            // text, expected, remaining text
            ("0", Ok((0..1, "0")), ""),
            ("1", Ok((0..1, "1")), ""),
            ("123", Ok((0..3, "123")), ""),
            ("1234567890", Ok((0..10, "1234567890")), ""),
            //
            ("0+", Ok((0..1, "0")), "+"),
            //
            // FIXME: ("0123", Err((0..1, "0")), "0123"),
            //
            ("-0", Err((0..0, "")), "-0"),
            ("-123", Err((0..0, "")), "-123"),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);

            let actual = scanner.scan_c_int_dec();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_c_int_hex() {
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

            let actual = scanner.scan_c_int_hex();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_c_int_oct() {
        let cases = [
            // text, expected, remaining text
            ("00", Ok((0..2, "00")), ""),
            ("07", Ok((0..2, "07")), ""),
            ("000", Ok((0..3, "000")), ""),
            ("077", Ok((0..3, "077")), ""),
            ("01234567", Ok((0..8, "01234567")), ""),
            //
            ("077+", Ok((0..3, "077")), "+"),
            //
            ("0", Err((0..1, "0")), "0"),
            //
            ("1", Err((0..0, "")), "1"),
            ("177", Err((0..0, "")), "177"),
            ("12345670", Err((0..0, "")), "12345670"),
            //
            ("-077", Err((0..0, "")), "-077"),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);

            let actual = scanner.scan_c_int_oct();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_c_float() {
        let cases = [
            // text, expected, remaining text
            ("1.", Ok((0..2, "1.")), ""),
            (".2", Ok((0..2, ".2")), ""),
            ("1.2", Ok((0..3, "1.2")), ""),
            //
            // FIXME: ("1.f", Ok((0..3, "1.f")), ""),
            // FIXME: (".2f", Ok((0..3, ".2f")), ""),
            // FIXME: ("1.2f", Ok((0..4, "1.2f")), ""),
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
            ("-1", Err((0..0, "")), "-1"),
            ("-1.", Err((0..0, "")), "-1."),
            ("-.2", Err((0..0, "")), "-.2"),
            ("-1.2", Err((0..0, "")), "-1.2"),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);

            let actual = scanner.scan_c_float();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_c_char() {
        let cases = [
            // text, expected, remaining text
            ("'a'", Ok((0..3, "'a'")), ""),
            ("'A'", Ok((0..3, "'A'")), ""),
            //
            // FIXME: ("'å'", Err((0..1, "'")), "å'"),
            // FIXME: ("'Å'", Err((0..1, "'")), "Å'"),
            // FIXME: ("'Á'", Err((0..1, "'")), "Å'"),
            // FIXME: ("'東'", Err((0..1, "'")), "Å'"),
            // FIXME: ("'🦀'", Err((0..1, "'")), "Å'"),
            //
            ("'\\0'", Ok((0..4, "'\\0'")), ""),
            ("'\\n'", Ok((0..4, "'\\n'")), ""),
            ("'\\77'", Ok((0..5, "'\\77'")), ""),
            ("'\\xF'", Ok((0..5, "'\\xF'")), ""),
            ("'\\xFF'", Ok((0..6, "'\\xFF'")), ""),
            //
            ("'", Err((0..1, "'")), "'"),
            ("'a", Err((0..2, "'a")), "'a"),
            ("'a '", Err((0..2, "'a")), "'a '"),
            //
            ("'\\xFFF'", Err((0..5, "'\\xFF")), "'\\xFFF'"),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);

            let actual = scanner.scan_c_char();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_c_string() {
        let cases = [
            // text, expected, remaining text
            ("\"\"", Ok((0..2, "\"\"")), ""),
            ("\" \"", Ok((0..3, "\" \"")), ""),
            ("\"Foo Bar\"", Ok((0..9, "\"Foo Bar\"")), ""),
            //
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
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);

            let actual = scanner.scan_c_string();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }
}
