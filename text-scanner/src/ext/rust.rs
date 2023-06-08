use crate::{Scanner, ScannerResult};

/// [`Scanner`] extension for scanning Rust tokens.
///
/// **Note:** When using the `scan_rust_*()` methods, the order they are
/// called matters.
pub trait RustScannerExt<'text> {
    fn scan_rust_line_comment(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_rust_block_comment(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_rust_identifier(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_rust_raw_identifier(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_rust_char(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_rust_string(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_rust_raw_string(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_rust_int_dec(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_rust_int_hex(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_rust_int_oct(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_rust_int_bin(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_rust_float(&mut self) -> ScannerResult<'text, &'text str>;
}

impl<'text> RustScannerExt<'text> for Scanner<'text> {
    // Reference: https://doc.rust-lang.org/reference/comments.html
    fn scan_rust_line_comment(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('/')?;
            scanner.accept_char('/')?;
            scanner.skip_until_char_any(&['\n', '\r']);
            Ok(())
        })
    }

    // Reference: https://doc.rust-lang.org/reference/comments.html
    fn scan_rust_block_comment(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('/')?;
            scanner.accept_char('*')?;
            let mut open = 1;
            loop {
                scanner.skip_until_char_any(&['*', '/']);

                match scanner.next() {
                    Ok((_r, '*')) => {
                        if let Ok((_r, '/')) = scanner.next() {
                            if open == 1 {
                                break;
                            }
                            open -= 1;
                        }
                    }
                    Ok((_r, '/')) => {
                        if let Ok((_r, '*')) = scanner.next() {
                            open += 1;
                        }
                    }
                    Ok((_r, _c)) => {}
                    Err(_) => break,
                }
            }
            Ok(())
        })
    }

    // Reference: https://doc.rust-lang.org/reference/identifiers.html
    fn scan_rust_identifier(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_if(|c| c.is_alphabetic() || (c == '_'))?;
            scanner.skip_while(|c| c.is_alphanumeric() || (c == '_'));
            Ok(())
        })
    }

    // Reference: https://doc.rust-lang.org/reference/identifiers.html
    fn scan_rust_raw_identifier(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('r')?;
            scanner.accept_char('#')?;
            scanner.scan_rust_identifier()?;
            Ok(())
        })
    }

    // Reference: https://doc.rust-lang.org/reference/tokens.html#character-literals
    fn scan_rust_char(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('\'')?;

            let (_r, c) = scanner.next()?;
            if c == '\\' {
                // Skip the next character as it is escaped
                // Note: Technically any character is not valid
                _ = scanner.next();
            }

            scanner.accept_char('\'')?;
            Ok(())
        })
    }

    // Reference: https://doc.rust-lang.org/reference/tokens.html#string-literals
    fn scan_rust_string(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('"')?;

            loop {
                scanner.skip_until_char_any(&['"', '\\']);
                match scanner.next() {
                    Ok((_r, '"')) => break,
                    Ok((_r, '\\')) => {
                        // Skip the next character as it is escaped
                        // Note: Technically any character is not valid
                        _ = scanner.next();
                    }
                    Ok(_) => unreachable!(),
                    Err(_) => break,
                }
            }

            Ok(())
        })
    }

    // Reference: https://doc.rust-lang.org/reference/tokens.html#raw-string-literals
    fn scan_rust_raw_string(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('r')?;
            let hashes = scanner.skip_while_char('#').0.len();
            scanner.accept_char('"')?;

            'scan: loop {
                scanner.skip_until_char('"');

                if scanner.next().is_err() {
                    break;
                }

                if hashes > 0 {
                    for _ in 0..hashes {
                        if scanner.accept_char('#').is_err() {
                            continue 'scan;
                        }
                    }

                    break;
                } else {
                    break;
                }
            }

            Ok(())
        })
    }

    // Reference: https://doc.rust-lang.org/reference/tokens.html#integer-literals
    fn scan_rust_int_dec(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_if(|c| c.is_ascii_digit())?;
            scanner.skip_while(|c| c.is_ascii_digit() || (c == '_'));
            Ok(())
        })
    }

    // Reference: https://doc.rust-lang.org/reference/tokens.html#integer-literals
    fn scan_rust_int_hex(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('0')?;
            scanner.accept_char('x')?;

            scanner.skip_while_char('_');
            scanner.accept_if(|c| c.is_ascii_hexdigit())?;

            scanner.skip_while(|c| c.is_ascii_hexdigit() || (c == '_'));

            Ok(())
        })
    }

    // Reference: https://doc.rust-lang.org/reference/tokens.html#integer-literals
    fn scan_rust_int_oct(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('0')?;
            scanner.accept_char('o')?;

            scanner.skip_while_char('_');
            scanner.accept_if(|c| matches!(c, '0'..='7'))?;

            scanner.skip_while(|c| matches!(c, '0'..='7' | '_'));

            Ok(())
        })
    }

    // Reference: https://doc.rust-lang.org/reference/tokens.html#integer-literals
    fn scan_rust_int_bin(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('0')?;
            scanner.accept_char('b')?;

            scanner.skip_while_char('_');
            scanner.accept_if(|c| matches!(c, '0' | '1'))?;

            scanner.skip_while(|c| matches!(c, '0' | '1' | '_'));

            Ok(())
        })
    }

    // Reference: https://doc.rust-lang.org/reference/tokens.html#floating-point-literals
    fn scan_rust_float(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.scan_rust_int_dec()?;
            scanner.accept_char('.')?;

            if scanner.scan_rust_int_dec().is_ok() {
                if scanner.accept_char_any(&['e', 'E']).is_ok() {
                    _ = scanner.accept_char_any(&['+', '-']);

                    scanner.accept_if(|c| c.is_ascii_digit() || (c == '_'))?;
                    scanner.skip_while(|c| c.is_ascii_digit() || (c == '_'));
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
    fn test_line_comments() {
        let code = "
            // Line Comment
            // Line Comment\r
            //! Inner Line Doc Comment
            /// Outer Line Doc Comment
            //
            //\t
            ///
        ";
        let mut scanner = Scanner::new(code);

        assert_eq!(scanner.skip_whitespace().0, 0..13);
        assert_eq!(
            scanner.scan_rust_line_comment(),
            Ok((13..28, "// Line Comment"))
        );

        assert_eq!(scanner.skip_whitespace().0, 28..41);
        assert_eq!(
            scanner.scan_rust_line_comment(),
            Ok((41..56, "// Line Comment"))
        );

        assert_eq!(scanner.skip_whitespace().0, 56..70);
        assert_eq!(
            scanner.scan_rust_line_comment(),
            Ok((70..96, "//! Inner Line Doc Comment"))
        );

        assert_eq!(scanner.skip_whitespace().0, 96..109);
        assert_eq!(
            scanner.scan_rust_line_comment(),
            Ok((109..135, "/// Outer Line Doc Comment"))
        );
        assert_eq!(scanner.skip_whitespace().0, 135..148);
        assert_eq!(scanner.scan_rust_line_comment(), Ok((148..150, "//")));

        assert_eq!(scanner.skip_whitespace().0, 150..163);
        assert_eq!(scanner.scan_rust_line_comment(), Ok((163..166, "//\t")));

        assert_eq!(scanner.skip_whitespace().0, 166..179);
        assert_eq!(scanner.scan_rust_line_comment(), Ok((179..182, "///")));

        assert_eq!(scanner.skip_whitespace().0, 182..191);
        assert_eq!(scanner.remaining_text(), "");
    }

    #[test]
    fn test_block_comments() {
        let code = "
            /* Single Line Block Comment */
            /* Two Line
            Block Comment */

            /*

            Multiline
            Block
            Comment

            */

            /*

            /* Nested
            // /* Block */
            Comment */

            */

            /**/
            /*
            */
            /**//*
            *//**/

            /* Unclosed Block Comment
        ";
        let mut scanner = Scanner::new(code);

        assert_eq!(scanner.skip_whitespace().0, 0..13);
        assert_eq!(
            scanner.scan_rust_block_comment(),
            Ok((13..44, "/* Single Line Block Comment */"))
        );

        assert_eq!(scanner.skip_whitespace().0, 44..57);
        assert_eq!(
            scanner.scan_rust_block_comment(),
            Ok((57..97, "/* Two Line\n            Block Comment */"))
        );

        assert_eq!(scanner.skip_whitespace().0, 97..111);
        assert_eq!(
            scanner.scan_rust_block_comment(),
            Ok((
                111..190,
                "/*\n\n            Multiline\n            Block\n            Comment\n\n            */"
            ))
        );

        assert_eq!(scanner.skip_whitespace().0, 190..204);
        assert_eq!(
            scanner.scan_rust_block_comment(),
            Ok((204..295, "/*\n\n            /* Nested\n            // /* Block */\n            Comment */\n\n            */"))
        );

        assert_eq!(scanner.skip_whitespace().0, 295..309);
        assert_eq!(scanner.scan_rust_block_comment(), Ok((309..313, "/**/")));

        assert_eq!(scanner.skip_whitespace().0, 313..326);
        assert_eq!(
            scanner.scan_rust_block_comment(),
            Ok((326..343, "/*\n            */"))
        );

        assert_eq!(scanner.skip_whitespace().0, 343..356);
        assert_eq!(scanner.scan_rust_block_comment(), Ok((356..360, "/**/")));

        assert_eq!(scanner.skip_whitespace().0, 360..360);
        assert_eq!(
            scanner.scan_rust_block_comment(),
            Ok((360..377, "/*\n            */"))
        );

        assert_eq!(scanner.skip_whitespace().0, 377..377);
        assert_eq!(scanner.scan_rust_block_comment(), Ok((377..381, "/**/")));

        assert_eq!(scanner.skip_whitespace().0, 381..395);
        assert_eq!(
            scanner.scan_rust_block_comment(),
            Ok((395..429, "/* Unclosed Block Comment\n        "))
        );

        assert_eq!(scanner.skip_whitespace().0, 429..429);
        assert_eq!(scanner.remaining_text(), "");
    }

    #[test]
    fn test_identifiers() {
        let cases = [
            // text, expected, remaining text
            ("_", Some("_"), ""),
            ("x", Some("x"), ""),
            ("foo", Some("foo"), ""),
            ("_bar", Some("_bar"), ""),
            ("foo_bar_baz__", Some("foo_bar_baz__"), ""),
            ("foo-bar", Some("foo"), "-bar"),
            ("2foo", None, "2foo"),
            ("+foo", None, "+foo"),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);

            let actual = scanner.scan_rust_identifier().map(|(_, ident)| ident).ok();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_raw_identifiers() {
        let cases = [
            // text, expected, remaining text
            ("r#x", Some("r#x"), ""),
            ("r#foo", Some("r#foo"), ""),
            ("r#_foo", Some("r#_foo"), ""),
            ("r#foo_bar_baz__", Some("r#foo_bar_baz__"), ""),
            ("r#type", Some("r#type"), ""),
            ("r#struct", Some("r#struct"), ""),
            // Warning: Technically Rust does not allow `r#_`. However, this implementation
            // only scans the raw identifier format, and does not verify the validity of the
            // raw identifiers
            ("r#_", Some("r#_"), ""),
            ("r", None, "r"),
            ("r#", None, "r#"),
            ("r#2", None, "r#2"),
            ("r#2foo", None, "r#2foo"),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);

            let actual = scanner
                .scan_rust_raw_identifier()
                .map(|(_, ident)| ident)
                .ok();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_strings() {
        let cases = [
            // text, expected, remaining text
            ("\"\"", Some("\"\""), ""),
            ("\"Hello World\"", Some("\"Hello World\""), ""),
            ("\"Hello\nWorld\"", Some("\"Hello\nWorld\""), ""),
            ("\"Hello\\nWorld\"", Some("\"Hello\\nWorld\""), ""),
            (r#""Hello \" World""#, Some(r#""Hello \" World""#), ""),
            (r#""Hello \\\" World""#, Some(r#""Hello \\\" World""#), ""),
            ("\"No Closing Quote", Some("\"No Closing Quote"), ""),
            (r#""Hello \\" World""#, Some(r#""Hello \\""#), " World\""),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);

            let actual = scanner.scan_rust_string().map(|(_, s)| s).ok();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_raw_strings() {
        let cases = [
            // text, expected, remaining text
            ("r\"\"", Some("r\"\""), ""),
            ("r#\"\"#", Some("r#\"\"#"), ""),
            ("r#\"\n\"\n\"\"#", Some("r#\"\n\"\n\"\"#"), ""),
            ("r#\"Hello \" World\"#", Some("r#\"Hello \" World\"#"), ""),
            (
                "r#####\"Foo #\"# Bar ####\"#### Baz\"#####",
                Some("r#####\"Foo #\"# Bar ####\"#### Baz\"#####"),
                "",
            ),
            (
                "r###\"Foo \"## Bar\" Baz",
                Some("r###\"Foo \"## Bar\" Baz"),
                "",
            ),
            ("r##\"\"#", Some("r##\"\"#"), ""),
            ("r#\"\"##", Some("r#\"\"#"), "#"),
            ("r\"Hello \" World\"", Some("r\"Hello \""), " World\""),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);

            let actual = scanner.scan_rust_raw_string().map(|(_, s)| s).ok();
            assert_eq!(actual, expected);

            assert_eq!(scanner.remaining_text(), remaining);
        }
    }
}
