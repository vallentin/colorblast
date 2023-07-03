use crate::{ext::RustScannerExt, CharExt, Scanner, ScannerResult};

/// Reference: <https://docs.swift.org/swift-book/documentation/the-swift-programming-language/lexicalstructure/#Keywords-and-Punctuation>
#[rustfmt::skip]
pub const SWIFT_KEYWORDS: &[&str] = &[
    // Keywords used in declarations
    "associatedtype", "class", "deinit", "enum", "extension", "fileprivate", "func",
    "import", "init", "inout", "internal", "let", "open", "operator", "private",
    "precedencegroup", "protocol", "public", "rethrows", "static", "struct", "subscript",
    "typealias", "var",
    // Keywords used in statements
    "break", "case", "catch", "continue", "default", "defer", "do", "else", "fallthrough",
    "for", "guard", "if", "in", "repeat", "return", "throw", "switch", "where", "while",
    // Keywords used in statements
    "Any", "as", "await", "catch", "false", "is", "nil", "rethrows", "self", "Self", "super",
    "throw", "throws", "true", "try",
    // Keywords used in patterns
    "_",
    // Keywords that begin with a number sign
    "#available", "#colorLiteral", "#elseif", "#else", "#endif", "#if", "#imageLiteral",
    "#keyPath", "#selector", "#sourceLocation",
    // Prior to Swift 5.9, the following keywords are reserved
    "#column", "#dsohandle", "#error", "#fileID", "#fileLiteral", "#filePath", "#file",
    "#function", "#line", "#warning",
    // Keywords reserved in particular contexts
    "associativity", "convenience", "didSet", "dynamic", "final", "get", "indirect", "infix",
    "lazy", "left", "mutating", "none", "nonmutating", "optional", "override", "postfix",
    "precedence", "prefix", "Protocol", "required", "right", "set", "some", "Type", "unowned",
    "weak", "willSet",
];

/// References:
/// - <https://docs.swift.org/swift-book/documentation/the-swift-programming-language/basicoperators/>
/// - <https://docs.swift.org/swift-book/documentation/the-swift-programming-language/advancedoperators/>
/// - <https://developer.apple.com/documentation/swift/operator-declarations>
#[rustfmt::skip]
pub const SWIFT_OPERATORS: &[&str] = &[
    "+", "-", "*", "/", "%", "|", "^",
    "=", "*=", "/=", "%=", "+=", "-=", "<<=", ">>=", "&=", "|=", "^=",
    "&", "&+", "&-", "&*",
    "&*=", "&+=", "&-=", "&<<=", "&>>=",
    "!", ".!", "~",
    "<", "<=", ">", ">=", "==", "!=", "===", "!==", "~=", ".<", ".<=", ".>", ".>=", ".==", ".!=",
    "&&", "||",
    "<<", ">>", "&<<", "&>>",
    "?", ":",
    "??",
    "..<", "...",
    ".",
    ".&", ".|", ".^",
    ".&=", ".|=", ".^=",
    // "is", "as", "as?", "as!",
];

pub const SWIFT_DELIMITERS: &[&str] = &["(", ")", "[", "]", "{", "}"];

/// [`Scanner`] extension for scanning Swift tokens.
///
/// See also [`SwiftStrExt`].
///
/// _Based on [Swift 5.9 beta]_.
///
/// [Swift 5.9 beta]: https://docs.swift.org/swift-book/documentation/the-swift-programming-language/lexicalstructure
pub trait SwiftScannerExt<'text>: crate::private::Sealed {
    fn scan_swift_line_comment(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_swift_block_comment(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_swift_identifier(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_swift_attribute_name(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_swift_keyword(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_swift_operator(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_swift_delimiter(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_swift_nil_literal(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_swift_boolean_literal(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_swift_int_literal(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_swift_int_dec_literal(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_swift_int_hex_literal(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_swift_int_oct_literal(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_swift_int_bin_literal(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_swift_float_literal(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_swift_float_dec_literal(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_swift_float_hex_literal(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_swift_string_literal(&mut self) -> ScannerResult<'text, &'text str>;

    fn scan_swift_regex_literal(&mut self) -> ScannerResult<'text, &'text str>;
}

impl<'text> SwiftScannerExt<'text> for Scanner<'text> {
    // Reference: https://docs.swift.org/swift-book/documentation/the-swift-programming-language/lexicalstructure#Whitespace-and-Comments
    #[inline]
    fn scan_swift_line_comment(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_rust_line_comment()
    }

    // Reference: https://docs.swift.org/swift-book/documentation/the-swift-programming-language/lexicalstructure#Whitespace-and-Comments
    #[inline]
    fn scan_swift_block_comment(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_rust_block_comment()
    }

    // Reference: https://docs.swift.org/swift-book/documentation/the-swift-programming-language/lexicalstructure#Identifiers
    fn scan_swift_identifier(&mut self) -> ScannerResult<'text, &'text str> {
        // TODO: Swift supports a wider range of unicode characters as identifiers, e.g. emojis
        self.scan_with(|scanner| {
            if scanner.accept_char('`').is_ok() {
                scan_swift_identifier(scanner)?;
                scanner.accept_char('`')?;
            } else {
                _ = scanner.accept_char('$');
                scan_swift_identifier(scanner)?;
            }
            Ok(())
        })
    }

    // Reference: https://docs.swift.org/swift-book/documentation/the-swift-programming-language/attributes/#unknown
    #[inline]
    fn scan_swift_attribute_name(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('@')?;
            scanner.scan_swift_identifier()?;
            Ok(())
        })
    }

    // Reference: https://docs.swift.org/swift-book/documentation/the-swift-programming-language/lexicalstructure/#Keywords-and-Punctuation
    fn scan_swift_keyword(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            let (r, ident) = scanner.scan_with(|scanner| {
                _ = scanner.accept_char('#');
                scan_swift_identifier(scanner)?;
                Ok(())
            })?;
            if ident.is_swift_keyword() {
                Ok(())
            } else {
                Err((r, ident))
            }
        })
    }

    // References:
    // - https://docs.swift.org/swift-book/documentation/the-swift-programming-language/basicoperators/
    // - https://docs.swift.org/swift-book/documentation/the-swift-programming-language/advancedoperators/
    // - https://developer.apple.com/documentation/swift/operator-declarations
    fn scan_swift_operator(&mut self) -> ScannerResult<'text, &'text str> {
        // TODO: Include `is as as? as!`?
        self.scan_with(|scanner| {
            let (first, c) = scanner.next()?;
            match c {
                // +  -  *  /  %
                // += -= *= /= %=
                // ^ ^=
                // ~ ~=
                '+' | '-' | '*' | '/' | '%' | '^' | '~' => {
                    _ = scanner.accept_char('=');
                }
                // = == ===
                // ! != !==
                '=' | '!' => {
                    _ = scanner.accept_char('=');
                    _ = scanner.accept_char('=');
                }
                // &
                // &&
                // &=
                // &+  &-  &*
                // &+= &-= &*=
                // &<< &<<=
                // &>> &>>=
                '&' => {
                    if let Ok((last, c)) = scanner.next() {
                        match c {
                            // && &=
                            '&' | '=' => {}
                            // &+  &-  &*
                            // &+= &-= &*=
                            '+' | '-' | '*' => {
                                _ = scanner.accept_char('=');
                            }
                            // &<< &<<=
                            // &>> &>>=
                            '<' | '>' => {
                                if scanner.accept_char(c).is_ok() {
                                    _ = scanner.accept_char('=');
                                } else {
                                    scanner.cursor = last.start;
                                }
                            }
                            _ => {
                                scanner.cursor = last.start;
                            }
                        }
                    }
                }
                // | |= ||
                '|' => {
                    _ = scanner.accept_char_any(&['|', '=']);
                }
                // < <= <<=
                // > >= >>=
                '<' | '>' => {
                    _ = scanner.accept_char(c);
                    _ = scanner.accept_char('=');
                }
                // .
                // .&  .|  .^  .!  .<  .>
                // .&= .|= .^= .!= .<= .>= .==
                // ... ..<
                '.' => {
                    if let Ok((last, c)) = scanner.next() {
                        match c {
                            // .&  .|  .^  .!  .<  .>
                            // .&= .|= .^= .!= .<= .>=
                            '&' | '|' | '^' | '!' | '<' | '>' => {
                                _ = scanner.accept_char('=');
                            }
                            // .==
                            '=' => {
                                if scanner.accept_char('=').is_err() {
                                    scanner.cursor = last.start;
                                }
                            }
                            // ... ..<
                            '.' => {
                                if scanner.accept_char_any(&['.', '<']).is_err() {
                                    scanner.cursor = last.start;
                                }
                            }
                            _ => {
                                scanner.cursor = last.start;
                            }
                        }
                    }
                }
                // ? ??
                '?' => {
                    _ = scanner.accept_char('?');
                }
                // :
                ':' => {}
                _ => return Err(scanner.ranged_text(first)),
            }
            Ok(())
        })
    }

    #[inline]
    fn scan_swift_delimiter(&mut self) -> ScannerResult<'text, &'text str> {
        let (r, _c) = self.accept_char_any(&['(', ')', '[', ']', '{', '}'])?;
        Ok(self.ranged_text(r))
    }

    // Reference: https://docs.swift.org/swift-book/documentation/the-swift-programming-language/lexicalstructure#Literals
    #[inline]
    fn scan_swift_nil_literal(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| match scan_swift_identifier(scanner)? {
            (_, "nil") => Ok(()),
            (r, s) => Err((r, s)),
        })
    }

    // Reference: https://docs.swift.org/swift-book/documentation/the-swift-programming-language/lexicalstructure#Literals
    #[inline]
    fn scan_swift_boolean_literal(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| match scan_swift_identifier(scanner)? {
            (_, "true" | "false") => Ok(()),
            (r, s) => Err((r, s)),
        })
    }

    // References:
    // - https://docs.swift.org/swift-book/documentation/the-swift-programming-language/lexicalstructure#Integer-Literals
    // - https://docs.swift.org/swift-book/documentation/the-swift-programming-language/thebasics#Numeric-Literals
    #[inline]
    fn scan_swift_int_literal(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_swift_int_hex_literal()
            .or_else(|_| self.scan_swift_int_oct_literal())
            .or_else(|_| self.scan_swift_int_bin_literal())
            .or_else(|_| self.scan_swift_int_dec_literal())
    }

    // References:
    // - https://docs.swift.org/swift-book/documentation/the-swift-programming-language/lexicalstructure#Integer-Literals
    // - https://docs.swift.org/swift-book/documentation/the-swift-programming-language/thebasics#Numeric-Literals
    #[inline]
    fn scan_swift_int_dec_literal(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_if_ext(char::is_ascii_digit)?;
            scanner.skip_while(|c| c.is_ascii_digit() || (c == '_'));
            Ok(())
        })
    }

    // References:
    // - https://docs.swift.org/swift-book/documentation/the-swift-programming-language/lexicalstructure#Integer-Literals
    // - https://docs.swift.org/swift-book/documentation/the-swift-programming-language/thebasics#Numeric-Literals
    #[inline]
    fn scan_swift_int_hex_literal(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('0')?;
            scanner.accept_char('x')?;

            scanner.skip_while_char('_');
            scanner.accept_if_ext(char::is_ascii_hexdigit)?;

            scanner.skip_while(|c| c.is_ascii_hexdigit() || (c == '_'));

            Ok(())
        })
    }

    // References:
    // - https://docs.swift.org/swift-book/documentation/the-swift-programming-language/lexicalstructure#Integer-Literals
    // - https://docs.swift.org/swift-book/documentation/the-swift-programming-language/thebasics#Numeric-Literals
    #[inline]
    fn scan_swift_int_oct_literal(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('0')?;
            scanner.accept_char('o')?;

            scanner.skip_while_char('_');
            scanner.accept_if(CharExt::is_ascii_octdigit)?;

            scanner.skip_while(|c| CharExt::is_ascii_octdigit(c) || (c == '_'));

            Ok(())
        })
    }

    // References:
    // - https://docs.swift.org/swift-book/documentation/the-swift-programming-language/lexicalstructure#Integer-Literals
    // - https://docs.swift.org/swift-book/documentation/the-swift-programming-language/thebasics#Numeric-Literals
    #[inline]
    fn scan_swift_int_bin_literal(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.accept_char('0')?;
            scanner.accept_char('b')?;

            scanner.skip_while_char('_');
            scanner.accept_if(CharExt::is_ascii_bindigit)?;

            scanner.skip_while(|c| c.is_ascii_bindigit() || (c == '_'));

            Ok(())
        })
    }

    // References:
    // - https://docs.swift.org/swift-book/documentation/the-swift-programming-language/lexicalstructure#Floating-Point-Literals
    // - https://docs.swift.org/swift-book/documentation/the-swift-programming-language/thebasics#Floating-Point-Numbers
    fn scan_swift_float_literal(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_swift_float_dec_literal()
            .or_else(|_| self.scan_swift_float_hex_literal())
    }

    // References:
    // - https://docs.swift.org/swift-book/documentation/the-swift-programming-language/lexicalstructure#Floating-Point-Literals
    // - https://docs.swift.org/swift-book/documentation/the-swift-programming-language/thebasics#Floating-Point-Numbers
    fn scan_swift_float_dec_literal(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.scan_swift_int_dec_literal()?;

            if scanner.accept_char('.').is_ok() {
                scanner.scan_swift_int_dec_literal()?;
            }

            if scanner.accept_char_any(&['e', 'E']).is_ok() {
                _ = scanner.accept_char_any(&['+', '-']);

                scanner.skip_while_char('_');
                scanner.scan_digits_or_underscores()?;
            }

            Ok(())
        })
    }

    // References:
    // - https://docs.swift.org/swift-book/documentation/the-swift-programming-language/lexicalstructure#Floating-Point-Literals
    // - https://docs.swift.org/swift-book/documentation/the-swift-programming-language/thebasics#Floating-Point-Numbers
    fn scan_swift_float_hex_literal(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            scanner.scan_swift_int_hex_literal()?;

            if scanner.accept_char('.').is_ok() {
                scanner.accept_if_ext(char::is_ascii_hexdigit)?;
                scanner.skip_while(|c| c.is_ascii_hexdigit() || (c == '_'));
            }

            scanner.accept_char_any(&['p', 'P'])?;

            _ = scanner.accept_char_any(&['+', '-']);
            scanner.scan_swift_int_dec_literal()?;

            Ok(())
        })
    }

    // Reference: https://docs.swift.org/swift-book/documentation/the-swift-programming-language/lexicalstructure#String-Literals
    #[inline]
    fn scan_swift_string_literal(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            let hashes = scanner.skip_while_char('#').0.len();
            scanner.accept_char('"')?;
            let is_triple_quote = scanner.accept_str("\"\"").is_ok();

            'scan: loop {
                scanner.skip_until_char_any(&['"', '\\']);
                match scanner.next() {
                    Ok((_r, '"')) => {
                        if is_triple_quote {
                            for _ in 0..2 {
                                if scanner.accept_char('"').is_err() {
                                    continue 'scan;
                                }
                            }
                        }

                        for _ in 0..hashes {
                            if scanner.accept_char('#').is_err() {
                                continue 'scan;
                            }
                        }

                        break;
                    }
                    Ok((_r, '\\')) => {
                        if let Ok((_, '(')) = scanner.next() {
                            let mut nested = 0;
                            loop {
                                scanner.skip_until_char_any(&['(', ')', '"', '\\']);
                                match scanner.next() {
                                    Ok((_r, '(')) => {
                                        nested += 1;
                                    }
                                    Ok((_r, ')')) => {
                                        if nested == 0 {
                                            continue 'scan;
                                        }
                                        nested -= 1;
                                    }
                                    Ok((r, '"')) => {
                                        scanner.cursor = r.start;
                                        scanner.scan_swift_string_literal()?;
                                    }
                                    Ok((_r, '\\')) => {
                                        // Skip the next character as it is escaped
                                        // Note: Technically any character is not valid
                                        _ = scanner.next();
                                    }
                                    Ok(_) => unreachable!(),
                                    Err(_) => break,
                                }
                            }
                        }
                        // else
                        // Skip the next character as it is escaped
                        // Note: Technically any character is not valid
                    }
                    Ok(_) => unreachable!(),
                    Err(_) => break,
                }
            }

            Ok(())
        })
    }

    // Reference: https://docs.swift.org/swift-book/documentation/the-swift-programming-language/lexicalstructure#Regular-Expression-Literals
    fn scan_swift_regex_literal(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            let hashes = scanner.skip_while_char('#').0.len();
            scanner.accept_char('/')?;

            if hashes == 0 {
                if let (_, '\\') = scanner.accept_if(|c| !c.is_whitespace() && (c != '/'))? {
                    // Skip the next character as it is escaped
                    // Note: Technically any character is not valid
                    _ = scanner.next();
                }
            }

            'scan: loop {
                scanner.skip_until_char_any(&['/', '\\']);
                match scanner.next() {
                    Ok((_r, '/')) => {
                        for _ in 0..hashes {
                            if scanner.accept_char('#').is_err() {
                                continue 'scan;
                            }
                        }

                        break;
                    }
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
}

// TODO: Swift supports a wider range of unicode characters as identifiers, e.g. emojis
#[inline]
fn scan_swift_identifier<'text>(scanner: &mut Scanner<'text>) -> ScannerResult<'text, &'text str> {
    let (first, _) = scanner.accept_if(|c| c.is_alphabetic() || (c == '_'))?;
    let (last, _) = scanner.skip_while(|c| c.is_alphanumeric() || (c == '_'));
    Ok(scanner.ranged_text(first.start..last.end))
}

/// [`str`] extension for checking if a `&str` is e.g. a Swift keyword.
pub trait SwiftStrExt {
    fn is_swift_keyword(&self) -> bool;

    fn is_swift_nil_literal(&self) -> bool;
    fn is_swift_boolean_literal(&self) -> bool;

    fn is_swift_operator(&self) -> bool;
    fn is_swift_delimiter(&self) -> bool;
}

impl SwiftStrExt for str {
    #[inline]
    fn is_swift_keyword(&self) -> bool {
        SWIFT_KEYWORDS.contains(&self)
    }

    #[inline]
    fn is_swift_nil_literal(&self) -> bool {
        self == "nil"
    }

    #[inline]
    fn is_swift_boolean_literal(&self) -> bool {
        (self == "true") || (self == "false")
    }

    #[inline]
    fn is_swift_operator(&self) -> bool {
        SWIFT_OPERATORS.contains(&self)
    }

    #[inline]
    fn is_swift_delimiter(&self) -> bool {
        SWIFT_DELIMITERS.contains(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_invalid_cases, assert_valid_cases};

    #[test]
    fn test_swift_identifiers() {
        let cases = ["foo", "`foo`", "$foo"];

        assert_valid_cases!(scan_swift_identifier, cases);
    }

    #[test]
    fn test_swift_identifiers_invalid() {
        let cases = ["", "0", "0foo"];

        assert_invalid_cases!(scan_swift_identifier, cases);
    }

    #[test]
    fn test_swift_keywords() {
        for &expected in SWIFT_KEYWORDS {
            let mut scanner = Scanner::new(expected);

            let actual = scanner.scan_swift_keyword().map(|(_r, kw)| kw);
            assert_eq!(actual, Ok(expected));

            assert!(expected.is_swift_keyword());
        }
    }

    #[test]
    fn test_swift_operators() {
        for &expected in SWIFT_OPERATORS {
            for remaining in ["", "remaining"] {
                let text = format!("{expected}{remaining}");
                let mut scanner = Scanner::new(&text);

                let actual = scanner.scan_swift_operator().map(|(_r, punct)| punct);
                assert_eq!(actual, Ok(expected));
                assert_eq!(scanner.remaining_text(), remaining);

                let actual = actual.unwrap();
                assert!(actual.is_swift_operator());
                assert!(!actual.is_swift_delimiter());
            }
        }
    }

    #[test]
    fn test_swift_delimiters() {
        for &expected in SWIFT_DELIMITERS {
            for remaining in ["", "remaining"] {
                let text = format!("{expected}{remaining}");
                let mut scanner = Scanner::new(&text);

                let actual = scanner.scan_swift_delimiter().map(|(_r, punct)| punct);
                assert_eq!(actual, Ok(expected));
                assert_eq!(scanner.remaining_text(), remaining);

                let actual = actual.unwrap();
                assert!(actual.is_swift_delimiter());
                assert!(!actual.is_swift_operator());
            }
        }
    }

    #[test]
    fn test_swift_nil_literals() {
        assert_eq!("nil".is_swift_nil_literal(), true);
        assert_eq!("nil".is_swift_boolean_literal(), false);
        assert_eq!("nil".is_swift_keyword(), true);

        assert_eq!("NIL".is_swift_nil_literal(), false);
        assert_eq!("NIL".is_swift_boolean_literal(), false);
        assert_eq!("NIL".is_swift_keyword(), false);

        assert_eq!("Nil".is_swift_nil_literal(), false);
        assert_eq!("Nil".is_swift_boolean_literal(), false);
        assert_eq!("Nil".is_swift_keyword(), false);
    }

    #[test]
    fn test_swift_boolean_literals() {
        assert_eq!("true".is_swift_boolean_literal(), true);
        assert_eq!("true".is_swift_nil_literal(), false);
        assert_eq!("true".is_swift_keyword(), true);

        assert_eq!("false".is_swift_boolean_literal(), true);
        assert_eq!("false".is_swift_nil_literal(), false);
        assert_eq!("false".is_swift_keyword(), true);
    }

    #[test]
    fn test_swift_int_dec_literals() {
        let cases = [
            "0",
            "2",
            "00",
            "0000",
            "0_00_0",
            "0_00_0__",
            "2147_483_648",
            "2147483648",
        ];

        assert_valid_cases!(scan_swift_int_dec_literal, cases);
        assert_valid_cases!(scan_swift_int_dec_literal, cases, "remaining");

        assert_valid_cases!(scan_swift_int_literal, cases);
        assert_valid_cases!(scan_swift_int_literal, cases, "remaining");
    }

    #[test]
    fn test_swift_int_dec_literals_invalid() {
        let cases = ["_0", "_10", "+1", "-123"];

        assert_invalid_cases!(scan_swift_int_dec_literal, cases);
        assert_invalid_cases!(scan_swift_int_literal, cases);
    }

    #[test]
    fn test_swift_int_hex_literals() {
        let cases = [
            "0xDada_Cafe",
            "0x00_FF__00_FF",
            "0x100000000",
            "0xC0B0",
            "0x7fff_ffff",
            "0x8000_0000",
            "0xffff_ffff",
            "0x7fff_ffff_ffff_ffff",
            "0x8000_0000_0000_0000",
            "0xffff_ffff_ffff_ffff",
        ];

        assert_valid_cases!(scan_swift_int_hex_literal, cases);
        assert_valid_cases!(scan_swift_int_hex_literal, cases, "remaining");

        assert_valid_cases!(scan_swift_int_literal, cases);
        assert_valid_cases!(scan_swift_int_literal, cases, "remaining");
    }

    #[test]
    fn test_swift_int_oct_literals() {
        let cases = [
            "0o372",
            "0o777",
            "0o177_7777_7777",
            "0o200_0000_0000",
            "0o377_7777_7777",
            "0o7_7777_7777_7777_7777_7777",
            "0o10_0000_0000_0000_0000_0000",
            "0o17_7777_7777_7777_7777_7777",
        ];

        assert_valid_cases!(scan_swift_int_oct_literal, cases);
        assert_valid_cases!(scan_swift_int_oct_literal, cases, "remaining");

        assert_valid_cases!(scan_swift_int_literal, cases);
        assert_valid_cases!(scan_swift_int_literal, cases, "remaining");
    }

    #[test]
    fn test_swift_int_bin_literals() {
        let cases = [
            "0b0",
            "0b1",
            //
            "0b0111_1111_1111_1111_1111_1111_1111_1111",
            "0b1000_0000_0000_0000_0000_0000_0000_0000",
            "0b1111_1111_1111_1111_1111_1111_1111_1111",
            "0b0111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111",
            "0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000",
            "0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111",
        ];

        assert_valid_cases!(scan_swift_int_bin_literal, cases);
        assert_valid_cases!(scan_swift_int_bin_literal, cases, "remaining");

        assert_valid_cases!(scan_swift_int_literal, cases);
        assert_valid_cases!(scan_swift_int_literal, cases, "remaining");
    }

    #[test]
    fn test_swift_float_literals() {
        let cases = [
            "0",
            "1",
            "123",
            "1.5",
            "2.0",
            "3.14",
            "1e1",
            "1e137",
            "1e+15",
            "1e-9",
            "6.022137e+23",
            //
            "000123.456",
            "1_000_000",
            "1_000_000.000_000_1",
        ];

        assert_valid_cases!(scan_swift_float_literal, cases);
        assert_valid_cases!(scan_swift_float_literal, cases, "remaining");

        assert_valid_cases!(scan_swift_float_dec_literal, cases);
        assert_valid_cases!(scan_swift_float_dec_literal, cases, "remaining");
    }

    #[test]
    fn test_swift_float_hex_literals() {
        let cases = [
            "0x0.0p0",
            "0xC.3p0",
            "0xF.Fp9",
            "0xFFFF.FFFFp9999",
            "0xFFFF.FFFFp+9999",
            "0xFFFF.FFFFp-9999",
        ];

        assert_valid_cases!(scan_swift_float_hex_literal, cases);
        assert_valid_cases!(scan_swift_float_hex_literal, cases, "remaining");
    }

    #[test]
    fn test_swift_string_literals() {
        let cases = [
            r#""""#,
            r#""Hello World""#,
            r#""Hello \" World""#,
            //
            r#""""""""#,
            r#""""Hello World""""#,
            r#""""Hello " World""""#,
            r#""""Hello
                World""""#,
            //
            r###"#"#"#"###,
            r###"#"Hello " World"#"###,
            r###"#"Hello "
                " World"#"###,
            //
            r#####"###"Line 1\###nLine 2"###"#####,
            //
            r#""1 2 3""#,
            r#""1 2 \(3)""#,
            r#""1 2 \(1 + 2)""#,
            r#""1 2 \(((1) + (2)))""#,
            r#""1 2 \(x)""#,
            r#""1 2 \("3")""#,
            r#""1 2 \(("3"))""#,
            r#""1 2 \("")""#,
            r#""1 2 \((""))""#,
            r#""1 2 \(("\""))""#,
            r#""1 2 \(("\("3\"4")"))""#,
        ];

        assert_valid_cases!(scan_swift_string_literal, cases);
        assert_valid_cases!(scan_swift_string_literal, cases, "remaining");
    }

    #[test]
    fn test_swift_regex_literals() {
        let cases = [
            "/foo/",
            "/\\d/",
            "/\\(/",
            "/\\//",
            "/a\\/b\\/c/",
            //
            "#/abc/#",
            "##/abc/##",
            //
            "#/ abc/#",
            "##/ abc/##",
            "#/ /#",
            "##/ /##",
            //
            "#/foo
               bar/#",
            "##/foo
                bar/##",
        ];

        assert_valid_cases!(scan_swift_regex_literal, cases);
        assert_valid_cases!(scan_swift_regex_literal, cases, "remaining");
    }

    #[test]
    fn test_swift_regex_literals_invalid() {
        let cases = ["//", "/ /"];

        assert_invalid_cases!(scan_swift_regex_literal, cases);
    }
}
