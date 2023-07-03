use crate::{ext::CScannerExt, CharExt, Scanner, ScannerResult};

/// Reference: <https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.9>
#[rustfmt::skip]
pub const JAVA_RESERVED_KEYWORDS: &[&str] = &[
    "_", "abstract", "assert", "boolean", "break", "byte", "case", "catch", "char", "class",
    "const", "continue", "default", "do", "double", "else", "enum", "extends", "final",
    "finally", "float", "for", "if", "goto", "implements", "import", "instanceof", "int",
    "interface", "long", "native", "new", "package", "private", "protected", "public",
    "return", "short", "static", "strictfp", "super", "switch", "synchronized", "this",
    "throw", "throws", "transient", "try", "void", "volatile", "while",
];

/// Reference: <https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.9>
#[rustfmt::skip]
pub const JAVA_CONTEXTUAL_KEYWORDS: &[&str] = &[
    "exports", "module", "non-sealed", "open", "opens", "permits", "provides", "record",
    "requires", "sealed", "to", "transitive", "uses", "var", "with", "yield",
];

/// Reference: <https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.12>
pub const JAVA_OPERATORS: &[&str] = &[
    "=", ">", "<", "!", "~", "?", ":", "->", "==", ">=", "<=", "!=", "&&", "||", "++", "--", "+",
    "-", "*", "/", "&", "|", "^", "%", "<<", ">>", ">>>", "+=", "-=", "*=", "/=", "&=", "|=", "^=",
    "%=", "<<=", ">>=", ">>>=",
];

/// Reference: <https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.11>
pub const JAVA_SEPARATORS: &[&str] = &[
    "(", ")", "{", "}", "[", "]", ";", ",", ".", "...", "@", "::",
];

/// [`Scanner`] extension for scanning Java tokens.
///
/// See also [`JavaStrExt`].
///
/// _Based on [Java SE 20 Edition]_.
///
/// [Java SE 20 Edition]: https://docs.oracle.com/javase/specs/jls/se20/html/index.html
pub trait JavaScannerExt<'text>: crate::private::Sealed {
    fn scan_java_line_comment(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_java_block_comment(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_java_identifier(&mut self) -> ScannerResult<'text, &'text str>;

    /// **Note:** `null`, `true`, and `false` are not keywords, but literals,
    /// see [`scan_java_null_literal()`] and [`scan_java_boolean_literal()`].
    ///
    /// [`scan_java_null_literal()`]: Self::scan_java_null_literal
    /// [`scan_java_boolean_literal()`]: Self::scan_java_boolean_literal
    fn scan_java_keyword(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_java_reserved_keyword(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_java_contextual_keyword(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_java_operator(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_java_separator(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_java_null_literal(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_java_boolean_literal(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_java_int_literal(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_java_int_dec_literal(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_java_int_hex_literal(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_java_int_oct_literal(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_java_int_bin_literal(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_java_float_literal(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_java_float_dec_literal(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_java_float_hex_literal(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_java_char_literal(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_java_string_literal(&mut self) -> ScannerResult<'text, &'text str>;
}

impl<'text> JavaScannerExt<'text> for Scanner<'text> {
    // Reference: https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.7
    #[inline]
    fn scan_java_line_comment(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_c_line_comment()
    }

    // Reference: https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.7
    #[inline]
    fn scan_java_block_comment(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_c_block_comment()
    }

    // Reference: https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.8
    #[inline]
    fn scan_java_identifier(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_c_identifier()
    }

    // Reference: https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.9
    #[inline]
    fn scan_java_keyword(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            let res = scanner.scan_java_contextual_keyword();
            match res {
                Ok(_) => Ok(()),
                Err((r, s)) if s.is_java_reserved_keyword() => {
                    scanner.cursor = r.end;
                    Ok(())
                }
                Err(res) => Err(res),
            }
        })
    }

    // Reference: https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.9
    #[inline]
    fn scan_java_reserved_keyword(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            let (r, s) = scanner.scan_java_identifier()?;
            if s.is_java_reserved_keyword() {
                Ok(())
            } else {
                Err((r, s))
            }
        })
    }

    // Reference: https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.9
    #[inline]
    fn scan_java_contextual_keyword(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            let (r, s) = scanner.scan_java_identifier()?;
            if s.is_java_contextual_keyword() {
                Ok(())
            } else if s == "non" {
                scanner.accept_char('-')?;
                scanner.accept_str("sealed")?;
                Ok(())
            } else {
                Err((r, s))
            }
        })
    }

    // Reference: https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.12
    fn scan_java_operator(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            let (r, c) = scanner.next()?;
            match c {
                // =  *  /  ^  %  !
                // == *= /= ^= %= !=
                '=' | '*' | '/' | '^' | '%' | '!' => {
                    _ = scanner.accept_char('=');
                }
                // + += ++
                '+' => {
                    _ = scanner.accept_char_any(&['=', '+']);
                }
                // - -= ->
                '-' => {
                    _ = scanner.accept_char_any(&['=', '-', '>']);
                }
                // & &= &&
                '&' => {
                    _ = scanner.accept_char_any(&['=', '&']);
                }
                // | |= ||
                '|' => {
                    _ = scanner.accept_char_any(&['=', '|']);
                }
                // <  <<
                // <= <<=
                '<' => {
                    _ = scanner.accept_char('<');
                    _ = scanner.accept_char('=');
                }
                // >  >>  >>>
                // >= >>= >>>=
                '>' => {
                    _ = scanner.accept_char('>');
                    _ = scanner.accept_char('>');
                    _ = scanner.accept_char('=');
                }
                // : ? ~
                ':' | '?' | '~' => {}
                _ => return Err(scanner.ranged_text(r)),
            }
            Ok(())
        })
    }

    // Reference: https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.11
    fn scan_java_separator(&mut self) -> ScannerResult<'text, &'text str> {
        let (r, c) = self.peek()?;
        let res = match c {
            '(' | ')' | '{' | '}' | '[' | ']' | ';' | ',' | '@' => {
                self.cursor = r.end;
                Ok(r)
            }
            ':' => match self.peek_nth(1) {
                Ok((last, ':')) => Ok(r.start..last.end),
                _ => Err(r),
            },
            '.' => {
                self.cursor = r.end;
                match self.peek_str(2) {
                    Ok((last, "..")) => Ok(r.start..last.end),
                    _ => Ok(r),
                }
            }
            _ => Err(r),
        };
        match res {
            Ok(r) => {
                self.cursor = r.end;
                Ok(self.ranged_text(r))
            }
            Err(r) => Err(self.ranged_text(r)),
        }
    }

    // Reference: https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.10.8
    #[inline]
    fn scan_java_null_literal(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            let (r, s) = scanner.scan_java_identifier()?;
            if s.is_java_null_literal() {
                Ok(())
            } else {
                Err((r, s))
            }
        })
    }

    // Reference: https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.10.3
    #[inline]
    fn scan_java_boolean_literal(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            let (r, s) = scanner.scan_java_identifier()?;
            if s.is_java_boolean_literal() {
                Ok(())
            } else {
                Err((r, s))
            }
        })
    }

    // Reference: https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.10.1
    #[inline]
    fn scan_java_int_literal(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_java_int_dec_literal()
            .or_else(|_| self.scan_java_int_hex_literal())
            .or_else(|_| self.scan_java_int_oct_literal())
            .or_else(|_| self.scan_java_int_bin_literal())
    }

    // Reference: https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.10.1
    #[inline]
    fn scan_java_int_dec_literal(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            match scanner.accept_if_ext(char::is_ascii_digit)? {
                (r, '0') => {
                    if scanner.peek().map_or(false, |(_r, c)| match c {
                        c if c.is_ascii_digit() => true,
                        'x' | 'X' | 'b' | 'B' | 'f' | 'F' | 'd' | 'D' => true,
                        _ => false,
                    }) {
                        return Err((r, "0"));
                    }
                }
                _ => {
                    scanner.skip_while(|c| c.is_ascii_non_zero_digit() || (c == '_'));
                }
            }

            _ = scanner.accept_char_any(&['l', 'L']);

            Ok(())
        })
    }

    // Reference: https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.10.1
    #[inline]
    fn scan_java_int_hex_literal(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('0')?;
            scanner.accept_char_any(&['x', 'X'])?;

            scanner.skip_while_char('_');
            scanner.accept_if_ext(char::is_ascii_hexdigit)?;

            scanner.skip_while(|c| c.is_ascii_hexdigit() || (c == '_'));

            _ = scanner.accept_char_any(&['l', 'L']);

            Ok(())
        })
    }

    // Reference: https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.10.1
    #[inline]
    fn scan_java_int_oct_literal(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('0')?;

            scanner.skip_while_char('_');
            scanner.accept_if(CharExt::is_ascii_octdigit)?;

            scanner.skip_while(|c| CharExt::is_ascii_octdigit(c) || (c == '_'));

            _ = scanner.accept_char_any(&['l', 'L']);

            Ok(())
        })
    }

    // Reference: https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.10.1
    #[inline]
    fn scan_java_int_bin_literal(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('0')?;
            scanner.accept_char_any(&['b', 'B'])?;

            scanner.skip_while_char('_');
            scanner.accept_if(CharExt::is_ascii_bindigit)?;

            scanner.skip_while(|c| c.is_ascii_bindigit() || (c == '_'));

            _ = scanner.accept_char_any(&['l', 'L']);

            Ok(())
        })
    }

    // Reference: https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.10.2
    #[inline]
    fn scan_java_float_literal(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_java_float_dec_literal()
            .or_else(|_| self.scan_java_float_hex_literal())
    }

    // Reference: https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.10.2
    fn scan_java_float_dec_literal(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            if scanner.accept_char('.').is_ok() {
                scanner.scan_digits_or_underscores()?;
            } else {
                scanner.scan_digits_or_underscores()?;
                if scanner.accept_char('.').is_ok() {
                    _ = scanner.scan_digits_or_underscores();
                }
            }

            if scanner.accept_char_any(&['e', 'E']).is_ok() {
                _ = scanner.accept_char_any(&['+', '-']);

                scanner.skip_while_char('_');
                scanner.scan_digits_or_underscores()?;
            }

            _ = scanner.accept_char_any(&['f', 'F', 'd', 'D']);

            Ok(())
        })
    }

    // Reference: https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.10.2
    fn scan_java_float_hex_literal(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('0')?;
            scanner.accept_char_any(&['x', 'X'])?;

            scanner.skip_while_char('_');
            scanner.accept_if_ext(char::is_ascii_hexdigit)?;
            scanner.skip_while(|c| c.is_ascii_hexdigit() || (c == '_'));

            scanner.accept_char('.')?;

            scanner.skip_while_char('_');
            if scanner.accept_if_ext(char::is_ascii_hexdigit).is_ok() {
                scanner.skip_while(|c| c.is_ascii_hexdigit() || (c == '_'));
            }

            scanner.accept_char_any(&['p', 'P'])?;

            _ = scanner.accept_char_any(&['+', '-']);
            scanner.skip_while_char('_');
            scanner.scan_digits_or_underscores()?;

            _ = scanner.accept_char_any(&['f', 'F', 'd', 'D']);

            Ok(())
        })
    }

    // Reference: https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.10.4
    fn scan_java_char_literal(&mut self) -> ScannerResult<'text, &'text str> {
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
                } else if c == 'u' {
                    scanner.accept_if_ext(char::is_ascii_hexdigit)?;
                    scanner.accept_if_ext(char::is_ascii_hexdigit)?;
                    scanner.accept_if_ext(char::is_ascii_hexdigit)?;
                    scanner.accept_if_ext(char::is_ascii_hexdigit)?;
                }
            }

            scanner.accept_char('\'')?;
            Ok(())
        })
    }

    // Reference: https://docs.oracle.com/javase/specs/jls/se20/html/jls-3.html#jls-3.10.5
    #[inline]
    fn scan_java_string_literal(&mut self) -> ScannerResult<'text, &'text str> {
        // TODO: It can scan `\uFFFF` but it is not handled correctly
        self.scan_c_string()
    }
}

/// [`str`] extension for checking if a `&str` is e.g. a Java keyword.
pub trait JavaStrExt {
    fn is_java_keyword(&self) -> bool;
    fn is_java_reserved_keyword(&self) -> bool;
    fn is_java_contextual_keyword(&self) -> bool;

    fn is_java_null_literal(&self) -> bool;
    fn is_java_boolean_literal(&self) -> bool;

    fn is_java_operator(&self) -> bool;
    fn is_java_separator(&self) -> bool;
}

impl JavaStrExt for str {
    #[inline]
    fn is_java_keyword(&self) -> bool {
        self.is_java_contextual_keyword() || self.is_java_reserved_keyword()
    }

    #[inline]
    fn is_java_reserved_keyword(&self) -> bool {
        JAVA_RESERVED_KEYWORDS.contains(&self)
    }

    #[inline]
    fn is_java_contextual_keyword(&self) -> bool {
        JAVA_CONTEXTUAL_KEYWORDS.contains(&self)
    }

    #[inline]
    fn is_java_null_literal(&self) -> bool {
        self == "null"
    }

    #[inline]
    fn is_java_boolean_literal(&self) -> bool {
        (self == "true") || (self == "false")
    }

    #[inline]
    fn is_java_operator(&self) -> bool {
        JAVA_OPERATORS.contains(&self)
    }

    #[inline]
    fn is_java_separator(&self) -> bool {
        JAVA_SEPARATORS.contains(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_invalid_cases, assert_valid_cases};

    #[test]
    fn test_java_keywords() {
        for &expected in JAVA_RESERVED_KEYWORDS
            .iter()
            .chain(JAVA_CONTEXTUAL_KEYWORDS)
        {
            let mut scanner = Scanner::new(expected);

            let actual = scanner.scan_java_keyword().map(|(_r, kw)| kw);
            assert_eq!(actual, Ok(expected));

            assert!(expected.is_java_keyword());
        }
    }

    #[test]
    fn test_java_reversed_keywords() {
        for &expected in JAVA_RESERVED_KEYWORDS {
            let mut scanner = Scanner::new(expected);

            let actual = scanner.scan_java_reserved_keyword().map(|(_r, kw)| kw);
            assert_eq!(actual, Ok(expected));

            let actual = actual.unwrap();
            assert!(actual.is_java_reserved_keyword());
            assert!(!actual.is_java_contextual_keyword());
        }
    }

    #[test]
    fn test_java_contextual_keywords() {
        for &expected in JAVA_CONTEXTUAL_KEYWORDS {
            let mut scanner = Scanner::new(expected);

            let actual = scanner.scan_java_contextual_keyword().map(|(_r, kw)| kw);
            assert_eq!(actual, Ok(expected));

            let actual = actual.unwrap();
            assert!(actual.is_java_contextual_keyword());
            assert!(!actual.is_java_reserved_keyword());
        }
    }

    #[test]
    fn test_java_operators() {
        for &expected in JAVA_OPERATORS {
            let mut scanner = Scanner::new(expected);

            let actual = scanner.scan_java_operator().map(|(_r, punct)| punct);
            assert_eq!(actual, Ok(expected));

            let actual = actual.unwrap();
            assert!(actual.is_java_operator());
            assert!(!actual.is_java_separator());
        }
    }

    #[test]
    fn test_java_separators() {
        for &expected in JAVA_SEPARATORS {
            let mut scanner = Scanner::new(expected);

            let actual = scanner.scan_java_separator().map(|(_r, punct)| punct);
            assert_eq!(actual, Ok(expected));

            let actual = actual.unwrap();
            assert!(actual.is_java_separator());
            assert!(!actual.is_java_operator());
        }
    }

    #[test]
    fn test_java_null_literals() {
        assert_eq!("null".is_java_null_literal(), true);
        assert_eq!("null".is_java_boolean_literal(), false);
        assert_eq!("null".is_java_keyword(), false);

        assert_eq!("NULL".is_java_null_literal(), false);
        assert_eq!("NULL".is_java_boolean_literal(), false);
        assert_eq!("NULL".is_java_keyword(), false);

        assert_eq!("Null".is_java_null_literal(), false);
        assert_eq!("Null".is_java_boolean_literal(), false);
        assert_eq!("Null".is_java_keyword(), false);
    }

    #[test]
    fn test_java_boolean_literals() {
        assert_eq!("true".is_java_boolean_literal(), true);
        assert_eq!("true".is_java_null_literal(), false);
        assert_eq!("true".is_java_keyword(), false);

        assert_eq!("false".is_java_boolean_literal(), true);
        assert_eq!("false".is_java_null_literal(), false);
        assert_eq!("false".is_java_keyword(), false);
    }

    #[test]
    fn test_java_int_dec_literals() {
        let cases = ["0", "2", "0l", "0L", "1996", "2_147_483_648L", "2147483648"];

        assert_valid_cases!(scan_java_int_dec_literal, cases);
        assert_valid_cases!(scan_java_int_dec_literal, cases, "remaining");

        assert_valid_cases!(scan_java_int_literal, cases);
        assert_valid_cases!(scan_java_int_literal, cases, "remaining");
    }

    #[test]
    fn test_java_int_dec_literals_invalid() {
        let cases = ["00", "0000", "_0", "_10", "+1", "-123"];

        assert_invalid_cases!(scan_java_int_dec_literal, cases);
    }

    #[test]
    fn test_java_int_hex_literals() {
        let cases = [
            "0xDada_Cafe",
            "0x00_FF__00_FF",
            "0x100000000L",
            "0xC0B0L",
            "0x7fff_ffff",
            "0x8000_0000",
            "0xffff_ffff",
            "0x7fff_ffff_ffff_ffffL",
            "0x8000_0000_0000_0000L",
            "0xffff_ffff_ffff_ffffL",
        ];

        assert_valid_cases!(scan_java_int_hex_literal, cases);
        assert_valid_cases!(scan_java_int_hex_literal, cases, "remaining");

        assert_valid_cases!(scan_java_int_literal, cases);
        assert_valid_cases!(scan_java_int_literal, cases, "remaining");
    }

    #[test]
    fn test_java_int_oct_literals() {
        let cases = [
            "0372",
            "0777L",
            "0177_7777_7777",
            "0200_0000_0000",
            "0377_7777_7777",
            "07_7777_7777_7777_7777_7777L",
            "010_0000_0000_0000_0000_0000L",
            "017_7777_7777_7777_7777_7777L",
        ];

        assert_valid_cases!(scan_java_int_oct_literal, cases);
        assert_valid_cases!(scan_java_int_oct_literal, cases, "remaining");

        assert_valid_cases!(scan_java_int_literal, cases);
        assert_valid_cases!(scan_java_int_literal, cases, "remaining");
    }

    #[test]
    fn test_java_int_bin_literals() {
        let cases = [
            "0b0",
            "0B0",
            "0b1",
            "0B1",
            //
            "0b0111_1111_1111_1111_1111_1111_1111_1111",
            "0b1000_0000_0000_0000_0000_0000_0000_0000",
            "0b1111_1111_1111_1111_1111_1111_1111_1111",
            "0b0111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111L",
            "0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000L",
            "0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111L",
        ];

        assert_valid_cases!(scan_java_int_bin_literal, cases);
        assert_valid_cases!(scan_java_int_bin_literal, cases, "remaining");

        assert_valid_cases!(scan_java_int_literal, cases);
        assert_valid_cases!(scan_java_int_literal, cases, "remaining");
    }

    #[test]
    fn test_java_float_literals() {
        let cases = [
            // Float Literals
            "1e1f",
            "2.f",
            ".3f",
            "0f",
            "3.14f",
            "6.022137e+23f",
            // Double Literals
            "1e1",
            "2.",
            ".3",
            "0.0",
            "3.14",
            "1e-9d",
            "1e137",
        ];

        assert_valid_cases!(scan_java_float_literal, cases);
        assert_valid_cases!(scan_java_float_literal, cases, "remaining");

        assert_valid_cases!(scan_java_float_dec_literal, cases);
        assert_valid_cases!(scan_java_float_dec_literal, cases, "remaining");
    }

    #[test]
    fn test_java_float_hex_literals() {
        let cases = [
            "0x1.fffffeP+127f",
            "0x0.000002P-126f",
            "0x1.0P-149f",
            "0x1.f_ffff_ffff_ffffP+1023",
            "0x0.0_0000_0000_0001P-1022",
        ];

        assert_valid_cases!(scan_java_float_hex_literal, cases);
        assert_valid_cases!(scan_java_float_hex_literal, cases, "remaining");
    }

    #[test]
    fn test_java_char_literals() {
        let cases = [
            "'a'",
            "'%'",
            "'\t'",
            "'\\\\'",
            "'\\''",
            "'\\u03a9'",
            "'\\uFFFF'",
            "'\\177'",
            "'™'",
        ];

        assert_valid_cases!(scan_java_char_literal, cases);
        assert_valid_cases!(scan_java_char_literal, cases, "remaining");
    }

    #[test]
    fn test_java_string_literals() {
        let cases = [
            "\"\"",
            "\"\\\"\"",
            "\"This string\"",
            "\"A\\u0000B\\u1111C\\uEEEED\\uFFFFE\"",
        ];

        assert_valid_cases!(scan_java_string_literal, cases);
        assert_valid_cases!(scan_java_string_literal, cases, "remaining");
    }
}
