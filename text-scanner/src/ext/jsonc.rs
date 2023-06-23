use crate::{ext::CScannerExt, Scanner, ScannerResult};

/// [`Scanner`] extension for scanning [JSON with Comments] tokens.
///
/// [JSON with Comments]: https://code.visualstudio.com/docs/languages/json#_json-with-comments
pub trait JsonCScannerExt<'text> {
    fn scan_jsonc_line_comment(&mut self) -> ScannerResult<'text, &'text str>;
    fn scan_jsonc_block_comment(&mut self) -> ScannerResult<'text, &'text str>;
}

impl<'text> JsonCScannerExt<'text> for Scanner<'text> {
    // Reference: https://code.visualstudio.com/docs/languages/json#_json-with-comments
    #[inline]
    fn scan_jsonc_line_comment(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_c_line_comment()
    }

    // Reference: https://code.visualstudio.com/docs/languages/json#_json-with-comments
    #[inline]
    fn scan_jsonc_block_comment(&mut self) -> ScannerResult<'text, &'text str> {
        self.scan_c_block_comment()
    }
}
