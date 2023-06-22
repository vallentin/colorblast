//! [`Scanner`] extensions for scanning e.g. [Rust], [C], [Python].
//!
//! See the [`any-lexer` crate] for lexer implementations using `text-scanner`,
//! e.g. [`RustLexer`] using [`RustScannerExt`].
//!
//! [`Scanner`]: crate::Scanner
//! [C]: CScannerExt
//! [Python]: PythonScannerExt
//! [Rust]: RustScannerExt
//!
//! [`any-lexer` crate]: https://docs.rs/any-lexer/*/any_lexer/
//! [`RustLexer`]: https://docs.rs/any-lexer/*/any_lexer/struct.RustLexer.html

mod c;
mod python;
mod rust;

pub use self::c::*;
pub use self::python::*;
pub use self::rust::*;
