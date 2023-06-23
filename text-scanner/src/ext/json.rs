use crate::{Scanner, ScannerResult};

/// [`Scanner`] extension for scanning JSON tokens.
pub trait JsonScannerExt<'text> {
    fn scan_json_string(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_json_number(&mut self) -> ScannerResult<'text, &'text str>;
}

impl<'text> JsonScannerExt<'text> for Scanner<'text> {
    // Reference: https://www.json.org/json-en.html
    fn scan_json_string(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            let (_r, _c) = scanner.accept_char('"')?;

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

    // Reference: https://www.json.org/json-en.html
    fn scan_json_number(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_with(|scanner| {
            let (_r, c) = scanner.accept_if(|c| c.is_ascii_digit() || (c == '-'))?;
            if c == '-' {
                scanner.accept_if(|c| c.is_ascii_digit())?;
            }
            scanner.skip_while(|c| c.is_ascii_digit());

            if scanner.accept_char('.').is_ok() {
                scanner.skip_while(|c| c.is_ascii_digit());
            }

            if scanner.accept_char_any(&['E', 'e']).is_ok() {
                _ = scanner.accept_char_any(&['+', '-']);
                scanner.skip_while(|c| c.is_ascii_digit());
            }

            Ok(())
        })
    }
}
