use crate::{ext::CScannerExt, Scanner, ScannerResult};

/// [`Scanner`] extension for scanning SCSS tokens.
///
/// See also [`CssScannerExt`].
///
/// [`CssScannerExt`]: super::CssScannerExt
pub trait ScssScannerExt<'text> {
    fn scan_scss_line_comment(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_scss_block_comment(&mut self) -> ScannerResult<'text, &'text str>;
}

impl<'text> ScssScannerExt<'text> for Scanner<'text> {
    #[inline]
    fn scan_scss_line_comment(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_c_line_comment()
    }

    #[inline]
    fn scan_scss_block_comment(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_c_block_comment()
    }
}
