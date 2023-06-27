use crate::{Scanner, ScannerResult};

/// [`Scanner`] extension for scanning CSS tokens.
///
/// See also [`ScssScannerExt`].
///
/// [`ScssScannerExt`]: super::ScssScannerExt
pub trait CssScannerExt<'text>: crate::private::Sealed {
    /// Scans a single [CSS block comment].
    ///
    /// **Note:** CSS block comments do **not** allow nested block comments.
    ///
    /// **Note:** This has the same lifetime as the original `text`,
    /// so the scanner can continue to be used while this exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_scanner::{ext::CssScannerExt, Scanner};
    ///
    /// let text = r#"
    ///   /* Block Comment */
    ///
    ///   /* Multi
    ///   // Line
    ///   /* Block
    ///      Comment */
    ///
    ///   /* Unterminated Block Comment
    /// "#;
    ///
    /// let comments = [
    ///     (3..22,  "/* Block Comment */"),
    ///     (26..71, "/* Multi\n  // Line\n  /* Block\n     Comment */"),
    ///     (75..105, "/* Unterminated Block Comment\n"),
    /// ];
    ///
    /// let mut scanner = Scanner::new(text);
    /// for comment in comments {
    ///     scanner.skip_whitespace();
    ///     assert_eq!(scanner.scan_css_block_comment(), Ok(comment));
    /// }
    ///
    /// # scanner.skip_whitespace();
    /// # assert_eq!(scanner.remaining_text(), "");
    /// ```
    ///
    /// [CSS block comment]: https://www.w3.org/TR/css-syntax-3/#comment-diagram
    fn scan_css_block_comment(&mut self) -> ScannerResult<'text, &'text str>;

    /// Scans a single [CSS identifier].
    ///
    /// **Note:** This has the same lifetime as the original `text`,
    /// so the scanner can continue to be used while this exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_scanner::{ext::CssScannerExt, Scanner};
    ///
    /// let text = r#"
    ///   foo
    ///   foo_bar
    ///   foo-bar
    ///   --foo
    /// "#;
    ///
    /// let idents = [
    ///     (3..6,   "foo"),
    ///     (9..16,  "foo_bar"),
    ///     (19..26, "foo-bar"),
    ///     (29..34, "--foo"),
    /// ];
    ///
    /// let mut scanner = Scanner::new(text);
    /// for ident in idents {
    ///     scanner.skip_whitespace();
    ///     assert_eq!(scanner.scan_css_identifier(), Ok(ident));
    /// }
    ///
    /// # scanner.skip_whitespace();
    /// # assert_eq!(scanner.remaining_text(), "");
    /// ```
    ///
    /// [CSS identifier]: https://www.w3.org/TR/css-syntax-3/#ident-token-diagram
    fn scan_css_identifier(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_css_at_keyword(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_css_hash(&mut self) -> ScannerResult<'text, &'text str>;

    /// Scans a single [CSS string].
    ///
    /// **Note:** This has the same lifetime as the original `text`,
    /// so the scanner can continue to be used while this exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_scanner::{ext::CssScannerExt, Scanner};
    ///
    /// let text = r#"
    ///   "Hello World"
    ///   'Hello World'
    ///
    ///   "Hello ' \" World"
    ///   'Hello \' " World'
    ///
    ///   "Unterminated String
    /// "#;
    ///
    /// let strings = [
    ///     (3..16,  r#""Hello World""#),
    ///     (19..32, r#"'Hello World'"#),
    ///     (36..54, r#""Hello ' \" World""#),
    ///     (57..75, r#"'Hello \' " World'"#),
    ///     (79..100, "\"Unterminated String\n"),
    /// ];
    ///
    /// let mut scanner = Scanner::new(text);
    /// for string in strings {
    ///     scanner.skip_whitespace();
    ///     assert_eq!(scanner.scan_css_string(), Ok(string));
    /// }
    ///
    /// # scanner.skip_whitespace();
    /// # assert_eq!(scanner.remaining_text(), "");
    /// ```
    ///
    /// [CSS string]: https://www.w3.org/TR/css-syntax-3/#string-token-diagram
    fn scan_css_string(&mut self) -> ScannerResult<'text, &'text str>;

    /// Scans a single [CSS number].
    ///
    /// **Note:** CSS numbers allow a unary `+` or `-` before the number,
    /// as opposed to other languages separating those into two different
    /// tokens.
    ///
    /// **Note:** This has the same lifetime as the original `text`,
    /// so the scanner can continue to be used while this exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_scanner::{ext::CssScannerExt, Scanner};
    ///
    /// let text = r#"
    ///   1
    ///   -2
    ///   +3
    ///   3.1415
    ///   +10.5E+100
    /// "#;
    ///
    /// let numbers = [
    ///     (3..4,   "1"),
    ///     (7..9,   "-2"),
    ///     (12..14, "+3"),
    ///     (17..23, "3.1415"),
    ///     (26..36, "+10.5E+100"),
    /// ];
    ///
    /// let mut scanner = Scanner::new(text);
    /// for num in numbers {
    ///     scanner.skip_whitespace();
    ///     assert_eq!(scanner.scan_css_number(), Ok(num));
    /// }
    ///
    /// # scanner.skip_whitespace();
    /// # assert_eq!(scanner.remaining_text(), "");
    /// ```
    ///
    /// [CSS number]: https://www.w3.org/TR/css-syntax-3/#number-token-diagram
    fn scan_css_number(&mut self) -> ScannerResult<'text, &'text str>;
}

impl<'text> CssScannerExt<'text> for Scanner<'text> {
    // Reference: https://www.w3.org/TR/css-syntax-3/#comment-diagram
    fn scan_css_block_comment(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_str("/*")?;

            loop {
                let (r, _) = scanner.skip_until_char('*');
                if r.is_empty() {
                    break;
                }

                // Safe to ignore as it is guaranteed to be `Ok`
                _ = scanner.accept_char('*');

                if scanner.accept_char('/').is_ok() {
                    break;
                }
            }

            Ok(())
        })
    }

    // Reference: https://www.w3.org/TR/css-syntax-3/#ident-token-diagram
    fn scan_css_identifier(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            if scanner.accept_char('-').is_ok() {
                if scanner.accept_char('-').is_ok() {
                } else {
                    scanner.accept_if(|c| c.is_alphabetic() || (c == '_'))?;
                }

                scanner.skip_while(|c| c.is_alphanumeric() || matches!(c, '_' | '-'));
            } else {
                scanner.accept_if(|c| c.is_alphabetic() || (c == '_'))?;
                scanner.skip_while(|c| c.is_alphanumeric() || matches!(c, '_' | '-'));
            }

            Ok(())
        })
    }

    // Reference: https://www.w3.org/TR/css-syntax-3/#at-keyword-token-diagram
    fn scan_css_at_keyword(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('@')?;
            scanner.scan_css_identifier()?;
            Ok(())
        })
    }

    // Reference: https://www.w3.org/TR/css-syntax-3/#hash-token-diagram
    fn scan_css_hash(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('#')?;
            scanner.accept_if(|c| c.is_alphanumeric() || matches!(c, '_' | '-'))?;
            scanner.skip_while(|c| c.is_alphanumeric() || matches!(c, '_' | '-'));
            Ok(())
        })
    }

    // Reference: https://www.w3.org/TR/css-syntax-3/#string-token-diagram
    fn scan_css_string(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            let (_r, quote) = scanner.accept_char_any(&['"', '\''])?;

            loop {
                scanner.skip_until(|c| (c == quote) || (c == '\\'));
                match scanner.next() {
                    Ok((_r, c)) if c == quote => break,
                    Ok((_r, '\\')) => {
                        // Skip the next character as it is escaped
                        _ = scanner.next();
                    }
                    Ok(_) => unreachable!(),
                    Err(_) => break,
                }
            }

            Ok(())
        })
    }

    // Reference: https://www.w3.org/TR/css-syntax-3/#number-token-diagram
    fn scan_css_number(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            _ = scanner.accept_char_any(&['+', '-']);

            if scanner.accept_char('.').is_ok() {
                scanner.accept_if_ext(char::is_ascii_digit)?;
                scanner.skip_while_ext(char::is_ascii_digit);
            } else {
                scanner.accept_if_ext(char::is_ascii_digit)?;
                scanner.skip_while_ext(char::is_ascii_digit);

                if scanner.accept_char('.').is_ok() {
                    scanner.accept_if_ext(char::is_ascii_digit)?;
                    scanner.skip_while_ext(char::is_ascii_digit);
                }
            }

            if scanner.accept_char_any(&['E', 'e']).is_ok() {
                _ = scanner.accept_char_any(&['+', '-']);
                scanner.accept_if_ext(char::is_ascii_digit)?;
                scanner.skip_while_ext(char::is_ascii_digit);
            }

            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_ident() {
        let cases = [
            ("x", Ok((0..1, "x")), ""),
            ("foo", Ok((0..3, "foo")), ""),
            ("foo123", Ok((0..6, "foo123")), ""),
            ("foo_123", Ok((0..7, "foo_123")), ""),
            ("foo-123", Ok((0..7, "foo-123")), ""),
            ("foo__123_", Ok((0..9, "foo__123_")), ""),
            ("foo--123-", Ok((0..9, "foo--123-")), ""),
            //
            ("_", Ok((0..1, "_")), ""),
            ("__", Ok((0..2, "__")), ""),
            ("_x", Ok((0..2, "_x")), ""),
            ("_1", Ok((0..2, "_1")), ""),
            ("--", Ok((0..2, "--")), ""),
            ("_-", Ok((0..2, "_-")), ""),
            ("-_", Ok((0..2, "-_")), ""),
            ("-x", Ok((0..2, "-x")), ""),
            ("--x", Ok((0..3, "--x")), ""),
            ("_foo", Ok((0..4, "_foo")), ""),
            ("__foo", Ok((0..5, "__foo")), ""),
            ("-foo", Ok((0..4, "-foo")), ""),
            ("--foo", Ok((0..5, "--foo")), ""),
            //
            ("--1", Ok((0..3, "--1")), ""),
            ("--1x", Ok((0..4, "--1x")), ""),
            ("--1+", Ok((0..3, "--1")), "+"),
            ("---1", Ok((0..4, "---1")), ""),
            ("---1x", Ok((0..5, "---1x")), ""),
            //
            ("æøå", Ok((0..6, "æøå")), ""),
            ("-æøå", Ok((0..7, "-æøå")), ""),
            ("--æøå", Ok((0..8, "--æøå")), ""),
            //
            ("x ", Ok((0..1, "x")), " "),
            ("_ ", Ok((0..1, "_")), " "),
            ("__ ", Ok((0..2, "__")), " "),
            ("-- ", Ok((0..2, "--")), " "),
            ("_- ", Ok((0..2, "_-")), " "),
            ("-_ ", Ok((0..2, "-_")), " "),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);
            assert_eq!(scanner.scan_css_identifier(), expected);
            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_css_ident_invalid() {
        let cases = [
            ("", Err((0..0, "")), ""),
            (" ", Err((0..0, "")), " "),
            ("-", Err((0..1, "-")), "-"),
            ("- ", Err((0..1, "-")), "- "),
            ("-1", Err((0..1, "-")), "-1"),
            ("-1x", Err((0..1, "-")), "-1x"),
            ("-1+", Err((0..1, "-")), "-1+"),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);
            assert_eq!(scanner.scan_css_identifier(), expected);
            assert_eq!(scanner.remaining_text(), remaining);
        }
    }

    #[test]
    fn test_css_num() {
        let cases = [
            ("", Err((0..0, "")), ""),
            (" ", Err((0..0, "")), " "),
            ("+", Err((0..1, "+")), "+"),
            ("-", Err((0..1, "-")), "-"),
            ("+ ", Err((0..1, "+")), "+ "),
            ("- ", Err((0..1, "-")), "- "),
            //
            ("1", Ok((0..1, "1")), ""),
            ("+1", Ok((0..2, "+1")), ""),
            ("-1", Ok((0..2, "-1")), ""),
            //
            ("1.2", Ok((0..3, "1.2")), ""),
            ("+1.2", Ok((0..4, "+1.2")), ""),
            ("-1.2", Ok((0..4, "-1.2")), ""),
            //
            (".1", Ok((0..2, ".1")), ""),
            ("+.1", Ok((0..3, "+.1")), ""),
            ("-.1", Ok((0..3, "-.1")), ""),
            //
            ("++", Err((0..1, "+")), "++"),
            ("--", Err((0..1, "-")), "--"),
            ("+-", Err((0..1, "+")), "+-"),
            ("-+", Err((0..1, "-")), "-+"),
            //
            ("++1", Err((0..1, "+")), "++1"),
            ("--1", Err((0..1, "-")), "--1"),
            ("+-1", Err((0..1, "+")), "+-1"),
            ("-+1", Err((0..1, "-")), "-+1"),
            //
            ("1E", Err((0..2, "1E")), "1E"),
            ("1EE", Err((0..2, "1E")), "1EE"),
            ("1E*", Err((0..2, "1E")), "1E*"),
            ("1E+", Err((0..3, "1E+")), "1E+"),
            ("1E+X", Err((0..3, "1E+")), "1E+X"),
        ];

        for (text, expected, remaining) in cases.iter().cloned() {
            let mut scanner = Scanner::new(text);
            assert_eq!(scanner.scan_css_number(), expected);
            assert_eq!(scanner.remaining_text(), remaining);
        }

        for (text, expected, remaining) in cases {
            if expected.is_err() {
                continue;
            }

            for e in ['E', 'e'] {
                for sign in ["", "+", "-"] {
                    let exponent = format!("{e}{sign}1");
                    let text = format!("{text}{exponent}");

                    let (r, expected) = expected.clone().unwrap();
                    let r = r.start..(r.end + exponent.len());
                    let expected = format!("{expected}{exponent}");
                    let expected = Ok((r, expected.as_str()));

                    let mut scanner = Scanner::new(&text);
                    assert_eq!(scanner.scan_css_number(), expected);
                    assert_eq!(scanner.remaining_text(), remaining);
                }
            }
        }
    }

    #[test]
    fn test_css_num_invalid() {
        let cases = [
            ("1E", Err((0..2, "1E")), "1E"),
            ("1E ", Err((0..2, "1E")), "1E "),
            ("1EE", Err((0..2, "1E")), "1EE"),
            ("1E*", Err((0..2, "1E")), "1E*"),
            ("1E+", Err((0..3, "1E+")), "1E+"),
            ("1E+ ", Err((0..3, "1E+")), "1E+ "),
            ("1E+X", Err((0..3, "1E+")), "1E+X"),
        ];

        for (text, expected, remaining) in cases {
            let mut scanner = Scanner::new(text);
            assert_eq!(scanner.scan_css_number(), expected);
            assert_eq!(scanner.remaining_text(), remaining);
        }
    }
}
