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
mod css;
mod java;
mod json;
mod jsonc;
mod lisp;
mod python;
mod rust;
mod scss;
mod swift;

pub use self::c::*;
pub use self::css::*;
pub use self::java::*;
pub use self::json::*;
pub use self::jsonc::*;
pub use self::lisp::*;
pub use self::python::*;
pub use self::rust::*;
pub use self::scss::*;
pub use self::swift::*;
