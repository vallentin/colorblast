use crate::{CharExt, Scanner, ScannerResult};

/// [`Scanner`] extension for scanning Rust tokens.
///
/// **Note:** When using the `scan_rust_*()` methods, the order they are
/// called matters.
pub trait RustScannerExt<'text> {
    /// Scans a single [Rust line comment].
    ///
    /// **Note:** This has the same lifetime as the original `text`,
    /// so the scanner can continue to be used while this exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_scanner::{ext::RustScannerExt, Scanner};
    ///
    /// let text = r#"
    ///   // Line Comment
    ///   //! Inner Doc Comment
    ///   /// Outer Doc Comment
    /// "#;
    ///
    /// let comments = [
    ///     (3..18,  "// Line Comment"),
    ///     (21..42, "//! Inner Doc Comment"),
    ///     (45..66, "/// Outer Doc Comment"),
    /// ];
    ///
    /// let mut scanner = Scanner::new(text);
    /// for comment in comments {
    ///     scanner.skip_whitespace();
    ///     assert_eq!(scanner.scan_rust_line_comment(), Ok(comment));
    /// }
    ///
    /// # scanner.skip_whitespace();
    /// # assert_eq!(scanner.remaining_text(), "");
    /// ```
    ///
    /// [Rust line comment]: https://doc.rust-lang.org/reference/comments.html
    fn scan_rust_line_comment(&mut self) -> ScannerResult<'text, &'text str>;

    /// Scans a single [Rust block comment].
    ///
    /// **Note:** Rust block comment **allow** nested block comments.
    ///
    /// **Note:** This has the same lifetime as the original `text`,
    /// so the scanner can continue to be used while this exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_scanner::{ext::RustScannerExt, Scanner};
    ///
    /// let text = r#"
    ///   /* Block Comment */
    ///
    ///   /* Multi
    ///   // Line
    ///      Block
    ///      Comment */
    ///
    ///   /* Multi
    ///   // Line /*
    ///      Nested
    ///   /* Block */
    ///      Comment */ */
    ///
    ///   /* Unterminated Block Comment
    /// "#;
    ///
    /// let comments = [
    ///     (3..22,    "/* Block Comment */"),
    ///     (26..71,   "/* Multi\n  // Line\n     Block\n     Comment */"),
    ///     (75..141,  "/* Multi\n  // Line /*\n     Nested\n  /* Block */\n     Comment */ */"),
    ///     (145..175, "/* Unterminated Block Comment\n"),
    /// ];
    ///
    /// let mut scanner = Scanner::new(text);
    /// for comment in comments {
    ///     scanner.skip_whitespace();
    ///     assert_eq!(scanner.scan_rust_block_comment(), Ok(comment));
    /// }
    ///
    /// # scanner.skip_whitespace();
    /// # assert_eq!(scanner.remaining_text(), "");
    /// ```
    ///
    /// [Rust block comment]: https://doc.rust-lang.org/reference/comments.html
    fn scan_rust_block_comment(&mut self) -> ScannerResult<'text, &'text str>;

    /// Scans a single [Rust identifier].
    ///
    /// **Note:** This **does not** differentiate between [Rust identifier]s
    /// and [Rust keyword]s. If needed manually check if the returned `Ok` string slice
    /// is a [Rust keyword] or not.
    ///
    /// **Note:** This has the same lifetime as the original `text`,
    /// so the scanner can continue to be used while this exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_scanner::{ext::RustScannerExt, Scanner};
    ///
    /// let text = r#"
    ///   foo
    ///   foo_bar
    ///   _foo_
    ///   æøå
    ///   ľúbiť
    ///   東京
    /// "#;
    ///
    /// let idents = [
    ///     (3..6,   "foo"),
    ///     (9..16,  "foo_bar"),
    ///     (19..24, "_foo_"),
    ///     (27..33, "æøå"),
    ///     (36..44, "ľúbiť"),
    ///     (47..53, "東京"),
    /// ];
    ///
    /// let mut scanner = Scanner::new(text);
    /// for ident in idents {
    ///     scanner.skip_whitespace();
    ///     assert_eq!(scanner.scan_rust_identifier(), Ok(ident));
    /// }
    ///
    /// # scanner.skip_whitespace();
    /// # assert_eq!(scanner.remaining_text(), "");
    /// ```
    ///
    /// [Rust identifier]: https://doc.rust-lang.org/reference/identifiers.html
    /// [Rust keyword]: https://doc.rust-lang.org/reference/keywords.html
    fn scan_rust_identifier(&mut self) -> ScannerResult<'text, &'text str>;

    /// Scans a single [raw Rust identifier].
    ///
    /// **Note:** This **does not** differentiate between [Rust identifier]s
    /// and [Rust keyword]s. If needed manually check if the returned `Ok` string slice
    /// is a [Rust keyword] or not.
    ///
    /// **Note:** This has the same lifetime as the original `text`,
    /// so the scanner can continue to be used while this exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_scanner::{ext::RustScannerExt, Scanner};
    ///
    /// let text = r#"
    ///   r#foo
    ///   r#type
    ///   r#while
    ///   r#æøå
    ///   r#ľúbiť
    ///   r#東京
    /// "#;
    ///
    /// let idents = [
    ///     (3..8,   "r#foo"),
    ///     (11..17, "r#type"),
    ///     (20..27, "r#while"),
    ///     (30..38, "r#æøå"),
    ///     (41..51, "r#ľúbiť"),
    ///     (54..62, "r#東京"),
    /// ];
    ///
    /// let mut scanner = Scanner::new(text);
    /// for ident in idents {
    ///     scanner.skip_whitespace();
    ///     assert_eq!(scanner.scan_rust_raw_identifier(), Ok(ident));
    /// }
    ///
    /// # scanner.skip_whitespace();
    /// # assert_eq!(scanner.remaining_text(), "");
    /// ```
    ///
    /// [raw Rust identifier]: https://doc.rust-lang.org/reference/identifiers.html
    /// [Rust identifier]: https://doc.rust-lang.org/reference/identifiers.html
    /// [Rust keyword]: https://doc.rust-lang.org/reference/keywords.html
    fn scan_rust_raw_identifier(&mut self) -> ScannerResult<'text, &'text str>;

    /// Scans a single [Rust character].
    ///
    /// **Note:** This has the same lifetime as the original `text`,
    /// so the scanner can continue to be used while this exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_scanner::{ext::RustScannerExt, Scanner};
    ///
    /// let text = r#"
    ///   'A'
    ///   'Æ'
    ///   'Á'
    ///   '東'
    ///   '🦀'
    ///
    ///   '"'
    ///   '\\'
    ///   '\''
    ///   '\n'
    ///   '\0'
    /// "#;
    ///
    /// let chars = [
    ///     (3..6,     "'A'"),
    ///     (9..13,    "'Æ'"),
    ///     (16..20,   "'Á'"),
    ///     (23..28,   "'東'"),
    ///     (31..37,   "'🦀'"),
    ///     (41..44,   "'\"'"),
    ///     (47..51,   "'\\\\'"),
    ///     (54..58,   "'\\''"),
    ///     (61..65, "'\\n'"),
    ///     (68..72, "'\\0'"),
    /// ];
    ///
    /// let mut scanner = Scanner::new(text);
    /// for c in chars {
    ///     scanner.skip_whitespace();
    ///     assert_eq!(scanner.scan_rust_char(), Ok(c));
    /// }
    ///
    /// # scanner.skip_whitespace();
    /// # assert_eq!(scanner.remaining_text(), "");
    /// ```
    ///
    /// [Rust character]: https://doc.rust-lang.org/reference/tokens.html#character-literals
    fn scan_rust_char(&mut self) -> ScannerResult<'text, &'text str>;

    /// Scans a single [Rust string].
    ///
    /// **Note:** This has the same lifetime as the original `text`,
    /// so the scanner can continue to be used while this exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_scanner::{ext::RustScannerExt, Scanner};
    ///
    /// let text = r#"
    ///   "Hello World"
    ///
    ///   "Rust strings
    ///    can span multiple
    ///    lines"
    ///
    ///   "Foo \" Bar"
    ///
    ///   "Unterminated String
    /// "#;
    ///
    /// let strings = [
    ///     (3..16,   "\"Hello World\""),
    ///     (20..64,  "\"Rust strings\n   can span multiple\n   lines\""),
    ///     (68..80,  "\"Foo \\\" Bar\""),
    ///     (84..105, "\"Unterminated String\n"),
    /// ];
    ///
    /// let mut scanner = Scanner::new(text);
    /// for string in strings {
    ///     scanner.skip_whitespace();
    ///     assert_eq!(scanner.scan_rust_string(), Ok(string));
    /// }
    ///
    /// # scanner.skip_whitespace();
    /// # assert_eq!(scanner.remaining_text(), "");
    /// ```
    ///
    /// [Rust string]: https://doc.rust-lang.org/reference/tokens.html#string-literals
    fn scan_rust_string(&mut self) -> ScannerResult<'text, &'text str>;

    /// Scans a single [raw Rust string].
    ///
    /// **Note:** This has the same lifetime as the original `text`,
    /// so the scanner can continue to be used while this exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_scanner::{ext::RustScannerExt, Scanner};
    ///
    /// let text = r#####"
    ///   r#"Hello World"#
    ///
    ///   r###"Raw Rust strings"
    ///       "can span multiple"
    ///       "lines"###
    ///
    ///   r##"Foo #"# Bar"##
    ///
    ///   r###"Unterminated String
    /// "#####;
    ///
    /// let raw_strings = [
    ///     (3..19,    "r#\"Hello World\"#"),
    ///     (23..88,   "r###\"Raw Rust strings\"\n      \"can span multiple\"\n      \"lines\"###"),
    ///     (92..110,  "r##\"Foo #\"# Bar\"##"),
    ///     (114..139, "r###\"Unterminated String\n"),
    /// ];
    ///
    /// let mut scanner = Scanner::new(text);
    /// for raw_string in raw_strings {
    ///     scanner.skip_whitespace();
    ///     assert_eq!(scanner.scan_rust_raw_string(), Ok(raw_string));
    /// }
    ///
    /// # scanner.skip_whitespace();
    /// # assert_eq!(scanner.remaining_text(), "");
    /// ```
    ///
    /// [raw Rust string]: https://doc.rust-lang.org/reference/tokens.html#string-literals
    fn scan_rust_raw_string(&mut self) -> ScannerResult<'text, &'text str>;

    /// Scans a single [Rust integer decimal literal].
    ///
    /// **Note:** Rust integer literals do not allow a sign in front
    /// of the literal, i.e. `-10` is two tokens `["-", "10"]`.
    ///
    /// **Note:** This has the same lifetime as the original `text`,
    /// so the scanner can continue to be used while this exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_scanner::{ext::RustScannerExt, Scanner};
    ///
    /// let text = r#"
    ///   0
    ///   123
    ///
    ///   1_
    ///   1__
    ///   1_2_3
    ///   1__2__3__
    /// "#;
    ///
    /// let integers = [
    ///     (3..4,   "0"),
    ///     (7..10,  "123"),
    ///     (14..16, "1_"),
    ///     (19..22, "1__"),
    ///     (25..30, "1_2_3"),
    ///     (33..42, "1__2__3__"),
    /// ];
    ///
    /// let mut scanner = Scanner::new(text);
    /// for integer in integers {
    ///     scanner.skip_whitespace();
    ///     assert_eq!(scanner.scan_rust_int_dec(), Ok(integer));
    /// }
    ///
    /// # scanner.skip_whitespace();
    /// # assert_eq!(scanner.remaining_text(), "");
    /// ```
    ///
    /// [Rust integer decimal literal]: https://doc.rust-lang.org/reference/tokens.html#integer-literals
    fn scan_rust_int_dec(&mut self) -> ScannerResult<'text, &'text str>;

    /// Scans a single [Rust integer hex literal].
    ///
    /// **Note:** This has the same lifetime as the original `text`,
    /// so the scanner can continue to be used while this exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_scanner::{ext::RustScannerExt, Scanner};
    ///
    /// let text = r#"
    ///   0x0
    ///   0xFF
    ///
    ///   0x_FF_FF_FF_FF_
    /// "#;
    ///
    /// let hex_integers = [
    ///     (3..6,   "0x0"),
    ///     (9..13,  "0xFF"),
    ///     (17..32, "0x_FF_FF_FF_FF_"),
    /// ];
    ///
    /// let mut scanner = Scanner::new(text);
    /// for hex_integer in hex_integers {
    ///     scanner.skip_whitespace();
    ///     assert_eq!(scanner.scan_rust_int_hex(), Ok(hex_integer));
    /// }
    ///
    /// # scanner.skip_whitespace();
    /// # assert_eq!(scanner.remaining_text(), "");
    /// ```
    ///
    /// [Rust integer hex literal]: https://doc.rust-lang.org/reference/tokens.html#integer-literals
    fn scan_rust_int_hex(&mut self) -> ScannerResult<'text, &'text str>;

    /// Scans a single [Rust integer octal literal].
    ///
    /// **Note:** This has the same lifetime as the original `text`,
    /// so the scanner can continue to be used while this exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_scanner::{ext::RustScannerExt, Scanner};
    ///
    /// let text = r#"
    ///   0o0
    ///   0o100
    ///
    ///   0o_1_0_0_
    /// "#;
    ///
    /// let oct_integers = [
    ///     (3..6,   "0o0"),
    ///     (9..14,  "0o100"),
    ///     (18..27, "0o_1_0_0_"),
    /// ];
    ///
    /// let mut scanner = Scanner::new(text);
    /// for oct_integer in oct_integers {
    ///     scanner.skip_whitespace();
    ///     assert_eq!(scanner.scan_rust_int_oct(), Ok(oct_integer));
    /// }
    ///
    /// # scanner.skip_whitespace();
    /// # assert_eq!(scanner.remaining_text(), "");
    /// ```
    ///
    /// [Rust integer octal literal]: https://doc.rust-lang.org/reference/tokens.html#integer-literals
    fn scan_rust_int_oct(&mut self) -> ScannerResult<'text, &'text str>;

    /// Scans a single [Rust integer binary literal].
    ///
    /// **Note:** This has the same lifetime as the original `text`,
    /// so the scanner can continue to be used while this exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_scanner::{ext::RustScannerExt, Scanner};
    ///
    /// let text = r#"
    ///   0b0
    ///   0b1
    ///   0b10
    ///   0b11
    ///   0b100
    ///
    ///   0b_1_0_0_
    /// "#;
    ///
    /// let bin_integers = [
    ///     (3..6,   "0b0"),
    ///     (9..12,  "0b1"),
    ///     (15..19, "0b10"),
    ///     (22..26, "0b11"),
    ///     (29..34, "0b100"),
    ///     (38..47, "0b_1_0_0_"),
    /// ];
    ///
    /// let mut scanner = Scanner::new(text);
    /// for bin_integer in bin_integers {
    ///     scanner.skip_whitespace();
    ///     assert_eq!(scanner.scan_rust_int_bin(), Ok(bin_integer));
    /// }
    ///
    /// # scanner.skip_whitespace();
    /// # assert_eq!(scanner.remaining_text(), "");
    /// ```
    ///
    /// [Rust integer binary literal]: https://doc.rust-lang.org/reference/tokens.html#integer-literals
    fn scan_rust_int_bin(&mut self) -> ScannerResult<'text, &'text str>;

    /// Scans a single [Rust floating-point literal].
    ///
    /// **Note:** This has the same lifetime as the original `text`,
    /// so the scanner can continue to be used while this exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use text_scanner::{ext::RustScannerExt, Scanner};
    ///
    /// let text = r#"
    ///   12.
    ///   12.34
    ///
    ///   12.
    ///   12.34
    ///
    ///   12.34E56
    ///   12.34E+56
    ///   12.34E-56
    ///
    ///   1_2_.
    ///   1_2_.3_4_
    ///
    ///   1_2_.3_4_E_5_6_
    ///   1_2_.3_4_E+_5_6_
    ///   1_2_.3_4_E-_5_6_
    /// "#;
    ///
    /// let floats = [
    ///     (3..6,     "12."),
    ///     (9..14,    "12.34"),
    ///     (18..21,   "12."),
    ///     (24..29,   "12.34"),
    ///     (33..41,   "12.34E56"),
    ///     (44..53,   "12.34E+56"),
    ///     (56..65,   "12.34E-56"),
    ///     (69..74,   "1_2_."),
    ///     (77..86,   "1_2_.3_4_"),
    ///     (90..105,  "1_2_.3_4_E_5_6_"),
    ///     (108..124, "1_2_.3_4_E+_5_6_"),
    ///     (127..143, "1_2_.3_4_E-_5_6_"),
    /// ];
    ///
    /// let mut scanner = Scanner::new(text);
    /// for float in floats {
    ///     scanner.skip_whitespace();
    ///     assert_eq!(scanner.scan_rust_float(), Ok(float));
    /// }
    ///
    /// # scanner.skip_whitespace();
    /// # assert_eq!(scanner.remaining_text(), "");
    /// ```
    ///
    /// [Rust floating-point literal]: https://doc.rust-lang.org/reference/tokens.html#floating-point-literals
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
            scanner.accept_if_ext(char::is_ascii_digit)?;
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
            scanner.accept_if_ext(char::is_ascii_hexdigit)?;

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
            scanner.accept_if(CharExt::is_ascii_octdigit)?;

            scanner.skip_while(|c| CharExt::is_ascii_octdigit(c) || (c == '_'));

            Ok(())
        })
    }

    // Reference: https://doc.rust-lang.org/reference/tokens.html#integer-literals
    fn scan_rust_int_bin(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('0')?;
            scanner.accept_char('b')?;

            scanner.skip_while_char('_');
            scanner.accept_if(CharExt::is_ascii_bindigit)?;

            scanner.skip_while(|c| c.is_ascii_bindigit() || (c == '_'));

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
