use crate::{Scanner, ScannerItem, ScannerResult};

/// Additionally, any [whitespace] is also considered delimiters
///
/// [whitespace]: char::is_whitespace
pub const LISP_LIKE_DELIMITERS: &[&str] = &[
    "(", ")", "[", "]", "{", "}", ";", /*"\"",*/ "’", "‘", "|",
];

/// Additionally, any [whitespace] is also considered delimiters
///
/// [whitespace]: char::is_whitespace
pub const LISP_LIKE_DELIMITER_CHARS: &[char] = &[
    '(', ')', '[', ']', '{', '}', ';', /*'"',*/ '’', '‘', '|',
];

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum LispLikeToken {
    Symbol,
    Int,
    Float,
    Ratio,
    String,
}

impl LispLikeToken {
    fn scan_token<'text>(
        scanner: &mut Scanner<'text>,
    ) -> Result<(ScannerItem<&'text str>, LispLikeToken), ScannerItem<&'text str>> {
        let (r, tok) = scanner.peeking(|scanner| {
            let start = scanner.cursor;

            let (r, mut tok) = match Self::scan_number_token(scanner) {
                Ok((r, tok)) => (r, Some(tok)),
                Err((r, _s)) => (r, None),
            };

            if r.is_empty() && scanner.accept_char('"').is_ok() {
                tok = Some(LispLikeToken::String);

                loop {
                    scanner.skip_until_char_any(&['"', '\\']);
                    match scanner.next() {
                        Ok((_r, '"')) => break,
                        Ok((_r, '\\')) => {
                            // Skip the next character as it is escaped
                            // Note: Technically any character might not be valid
                            _ = scanner.next();
                        }
                        Ok(_) => unreachable!(),
                        Err(_) => break,
                    }
                }
            } else {
                loop {
                    _ = scanner.skip_while(|c| is_valid_symbol_char(c) && (c != '\\'));

                    if scanner.accept_char('\\').is_ok() {
                        // Skip the next character as it is escaped
                        // Note: Technically any character is not valid
                        _ = scanner.next();
                    } else {
                        break;
                    }
                }

                if tok.is_none() || (scanner.cursor > r.end) {
                    tok = Some(LispLikeToken::Symbol);
                }
            }

            let r = start..scanner.cursor;
            if r.is_empty() {
                Err(scanner.ranged_text(r))
            } else {
                Ok((r, tok))
            }
        })?;

        scanner.cursor = r.end;

        let (r, s) = scanner.ranged_text(r);
        match tok {
            Some(tok) => Ok(((r, s), tok)),
            None => Err((r, s)),
        }
    }

    /// # Grammar
    ///
    /// The following [EBNF] grammar represents what this method accepts:
    ///
    /// _The char immediately following `NumericToken` must be a delimiter,
    /// otherwise the callee should return `Err` overall._
    ///
    /// ```text
    /// NumericToken   ::= Int | Float | Ratio
    /// Int            ::= Sign? Digit+
    /// Float          ::= Sign? ( Digit+ '.' Digit* |
    ///                            Digit* '.' Digit+ ) Exponent?
    /// Exponent       ::= ExponentMarker Sign? Digit+
    /// ExponentMarker ::= [eEdDfFlLsS]
    /// Ratio          ::= Sign? Digit+ '/' Digit+
    /// Digit          ::= [0-9]
    /// Sign           ::= '+' | '-'
    /// ```
    ///
    /// [EBNF]: https://www.w3.org/TR/REC-xml/#sec-notation
    fn scan_number_token<'text>(
        scanner: &mut Scanner<'text>,
    ) -> ScannerResult<'text, LispLikeToken> {
        // References:
        // - http://www.lispworks.com/documentation/HyperSpec/Body/02_ca.htm
        // - http://www.lispworks.com/documentation/HyperSpec/Body/26_glo_s.htm#sign
        // - http://www.lispworks.com/documentation/HyperSpec/Body/26_glo_e.htm#exponent_marker
        let (r, tok) = scanner.peeking(|scanner| {
            let start = scanner.cursor;
            _ = scanner.accept_char_any(&['+', '-']);

            let mut tok = LispLikeToken::Int;

            if scanner.accept_char('.').is_ok() {
                scanner.accept_if_ext(char::is_ascii_digit)?;
                scanner.skip_while_ext(char::is_ascii_digit);

                tok = LispLikeToken::Float;
            } else {
                scanner.accept_if_ext(char::is_ascii_digit)?;
                scanner.skip_while_ext(char::is_ascii_digit);

                if scanner.accept_char('.').is_ok() {
                    scanner.skip_while_ext(char::is_ascii_digit);

                    tok = LispLikeToken::Float;
                } else if scanner.accept_char('/').is_ok() {
                    scanner.accept_if_ext(char::is_ascii_digit)?;
                    scanner.skip_while_ext(char::is_ascii_digit);

                    tok = LispLikeToken::Ratio;
                }
            }

            if matches!(tok, LispLikeToken::Int | LispLikeToken::Float)
                && scanner
                    .accept_char_any(&['e', 'E', 'd', 'D', 'f', 'F', 'l', 'L', 's', 'S'])
                    .is_ok()
            {
                _ = scanner.accept_char_any(&['+', '-']);
                scanner.accept_if_ext(char::is_ascii_digit)?;
                scanner.skip_while_ext(char::is_ascii_digit);

                tok = LispLikeToken::Float;
            }

            Ok((start..scanner.cursor, tok))
        })?;

        scanner.cursor = r.end;

        Ok((r, tok))
    }
}

/// [`Scanner`] extension for scanning Lisp-like tokens.
///
/// See also [`LispLikeCharExt`] and [`LispLikeStrExt`].
///
/// _Based on [Scheme], [Emacs Lisp], and [Common Lisp HyperSpec]_.
///
/// [Scheme]: https://www.gnu.org/software/mit-scheme/documentation/stable/mit-scheme-ref.pdf
/// [Emacs Lisp]: https://www.gnu.org/software/emacs/manual/html_mono/elisp.html
/// [Common Lisp HyperSpec]: http://www.lispworks.com/documentation/HyperSpec/Front/index.htm
pub trait LispLikeScannerExt<'text>: crate::private::Sealed {
    fn scan_lisp_like_delimiter(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_lisp_like_token(
        &mut self,
    ) -> Result<(ScannerItem<&'text str>, LispLikeToken), ScannerItem<&'text str>>;

    fn scan_lisp_like_symbol_name(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_lisp_like_number_literal(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_lisp_like_int_literal(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_lisp_like_float_literal(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_lisp_like_ratio_literal(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_lisp_like_string_literal(&mut self) -> ScannerResult<'text, &'text str>;
}

impl<'text> LispLikeScannerExt<'text> for Scanner<'text> {
    #[inline]
    fn scan_lisp_like_delimiter(&mut self) -> ScannerResult<'text, &'text str> {
        let (r, _c) = self.accept_if(|c| c.is_lisp_like_delimiter())?;
        Ok(self.ranged_text(r))
    }

    #[inline]
    fn scan_lisp_like_token(
        &mut self,
    ) -> Result<(ScannerItem<&'text str>, LispLikeToken), ScannerItem<&'text str>> {
        LispLikeToken::scan_token(self)
    }

    #[inline]
    fn scan_lisp_like_symbol_name(&mut self) -> ScannerResult<'text, &'text str> {
        let (r, s) = self.peeking(|scanner| {
            let ((r, s), tok) = scanner.scan_lisp_like_token()?;
            match tok {
                LispLikeToken::Symbol => Ok((r, s)),
                _ => Err((r, s)),
            }
        })?;
        self.cursor = r.end;
        Ok((r, s))
    }

    #[inline]
    fn scan_lisp_like_number_literal(&mut self) -> ScannerResult<'text, &'text str> {
        let (r, s) = self.peeking(|scanner| {
            let ((r, s), tok) = scanner.scan_lisp_like_token()?;
            match tok {
                LispLikeToken::Int | LispLikeToken::Float => Ok((r, s)),
                _ => Err((r, s)),
            }
        })?;
        self.cursor = r.end;
        Ok((r, s))
    }

    #[inline]
    fn scan_lisp_like_int_literal(&mut self) -> ScannerResult<'text, &'text str> {
        let (r, s) = self.peeking(|scanner| {
            let ((r, s), tok) = scanner.scan_lisp_like_token()?;
            match tok {
                LispLikeToken::Int => Ok((r, s)),
                _ => Err((r, s)),
            }
        })?;
        self.cursor = r.end;
        Ok((r, s))
    }

    #[inline]
    fn scan_lisp_like_float_literal(&mut self) -> ScannerResult<'text, &'text str> {
        let (r, s) = self.peeking(|scanner| {
            let ((r, s), tok) = scanner.scan_lisp_like_token()?;
            match tok {
                LispLikeToken::Float => Ok((r, s)),
                _ => Err((r, s)),
            }
        })?;
        self.cursor = r.end;
        Ok((r, s))
    }

    #[inline]
    fn scan_lisp_like_ratio_literal(&mut self) -> ScannerResult<'text, &'text str> {
        let (r, s) = self.peeking(|scanner| {
            let ((r, s), tok) = scanner.scan_lisp_like_token()?;
            match tok {
                LispLikeToken::Ratio => Ok((r, s)),
                _ => Err((r, s)),
            }
        })?;
        self.cursor = r.end;
        Ok((r, s))
    }

    #[inline]
    fn scan_lisp_like_string_literal(&mut self) -> ScannerResult<'text, &'text str> {
        let (r, s) = self.peeking(|scanner| {
            let ((r, s), tok) = scanner.scan_lisp_like_token()?;
            match tok {
                LispLikeToken::String => Ok((r, s)),
                _ => Err((r, s)),
            }
        })?;
        self.cursor = r.end;
        Ok((r, s))
    }
}

/// [`char`] extension for checking if a `char` is e.g. a Lisp-like delimiter.
pub trait LispLikeCharExt {
    #[allow(clippy::wrong_self_convention)]
    fn is_lisp_like_delimiter(self) -> bool;
}

impl LispLikeCharExt for char {
    #[inline]
    fn is_lisp_like_delimiter(self) -> bool {
        LISP_LIKE_DELIMITER_CHARS.contains(&self) || self.is_whitespace()
    }
}

/// [`str`] extension for checking if a `&str` is e.g. a Lisp-like delimiter.
pub trait LispLikeStrExt {
    fn is_lisp_like_delimiter(&self) -> bool;
}

impl LispLikeStrExt for str {
    #[inline]
    fn is_lisp_like_delimiter(&self) -> bool {
        self.chars().all(char::is_lisp_like_delimiter)
    }
}

fn is_valid_symbol_char(c: char) -> bool {
    match c {
        '(' | ')' | '[' | ']' | '{' | '}' => false,
        ';' | '"' | '’' | '‘' | '|' => false,
        c if c.is_whitespace() => false,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_invalid_cases, assert_valid_cases};

    #[test]
    fn test_lisp_like_delimiters() {
        for &expected in LISP_LIKE_DELIMITERS {
            for remaining in ["", "remaining"] {
                let text = format!("{expected}{remaining}");
                let mut scanner = Scanner::new(&text);

                let actual = scanner.scan_lisp_like_delimiter().map(|(_r, punct)| punct);
                assert_eq!(actual, Ok(expected));
                assert_eq!(scanner.remaining_text(), remaining);

                let actual = actual.unwrap();
                assert!(actual.is_lisp_like_delimiter());
            }
        }
    }

    #[test]
    fn test_lisp_like_symbol_names() {
        let cases = [
            // https://www.gnu.org/software/emacs/manual/html_mono/elisp.html#Symbol-Type
            "foo",
            "FOO",
            "1+",
            "\\+1",
            "\\(*\\ 1\\ 2\\)",
            "+-*/_~!@$%^&=:<>",
            // TODO: "+-*/_~!@$%^&=:<>{}",
            // http://www.lispworks.com/documentation/HyperSpec/Body/f_symb_2.htm#symbol-name
            "'temp",
            ":start",
            // http://www.lispworks.com/documentation/HyperSpec/Body/f_kwdp.htm#keywordp
            "'elephant",
            ":test",
            "':test",
            "'&optional",
            // http://www.lispworks.com/documentation/HyperSpec/Body/02_cd.htm
            "FROBBOZ",
            "frobboz",
            "fRObBoz",
            "unwind-protect",
            "+$",
            "pascal_style",
            "file.rel.43",
            "\\(",
            "\\+1",
            "+\\1",
            "\\frobboz",
            "3.14159265\\s0",
            "3.14159265\\S0",
            // https://docs.scheme.org/guide/common-lisp/
            "<",
            "list?",
            "proper-list?",
            "string<?",
            // http://www.lispworks.com/documentation/HyperSpec/Body/02_ca.htm
            "1/",
            "/2",
            "1/a",
            "a/2",
            "123/+456",
            "123/-456",
        ];

        assert_valid_cases!(scan_lisp_like_symbol_name, cases);
        assert_valid_cases!(scan_lisp_like_symbol_name, cases, "|");
    }

    #[test]
    fn test_lisp_like_symbol_names_invalid() {
        let cases = ["", "|"];

        assert_invalid_cases!(scan_lisp_like_symbol_name, cases);
    }

    #[test]
    fn test_lisp_like_int_literals() {
        let cases = [
            // https://www.gnu.org/software/emacs/manual/html_mono/elisp.html#Integer-Type
            "0", "1", "123", "00000", "+1", "-1",
            // TODO: "1.",
        ];

        assert_valid_cases!(scan_lisp_like_int_literal, cases);
        assert_valid_cases!(scan_lisp_like_int_literal, cases, "|");

        assert_valid_cases!(scan_lisp_like_number_literal, cases);
        assert_valid_cases!(scan_lisp_like_number_literal, cases, "|");
    }

    #[test]
    fn test_lisp_like_float_literals() {
        let cases = [
            // https://www.gnu.org/software/emacs/manual/html_mono/elisp.html#Floating_002dPoint-Type
            "1500.0",
            "+15e2",
            "15.0e+2",
            "+1500000e-3",
            ".15e4",
        ];

        assert_valid_cases!(scan_lisp_like_float_literal, cases);
        assert_valid_cases!(scan_lisp_like_float_literal, cases, "|");

        assert_valid_cases!(scan_lisp_like_number_literal, cases);
        assert_valid_cases!(scan_lisp_like_number_literal, cases, "|");
    }

    #[test]
    fn test_lisp_like_ratio_literals() {
        let cases = [
            // http://www.lispworks.com/documentation/HyperSpec/Body/02_ca.htm
            "1/2", "123/456", "+123/456", "-123/456",
        ];

        assert_valid_cases!(scan_lisp_like_ratio_literal, cases);
        assert_valid_cases!(scan_lisp_like_ratio_literal, cases, "|");
    }

    #[test]
    fn test_lisp_like_ratio_literals_invalid() {
        let cases = [
            // http://www.lispworks.com/documentation/HyperSpec/Body/02_ca.htm
            "1/", "/2", "1/a", "a/2", "123/+456", "123/-456",
        ];

        assert_invalid_cases!(scan_lisp_like_ratio_literal, cases);
    }

    #[test]
    fn test_lisp_like_string_literals() {
        let cases = [
            // https://www.gnu.org/software/emacs/manual/html_mono/elisp.html#String-Type
            "\"\"",
            "\"Hello World\"",
            "\"\n\"",
            "\"Hello\nWorld\"",
        ];

        assert_valid_cases!(scan_lisp_like_string_literal, cases);
        assert_valid_cases!(scan_lisp_like_string_literal, cases, "|");
    }
}
