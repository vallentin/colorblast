//! [`Scanner`] extensions for scanning e.g. [Rust], [C].
//!
//! [`Scanner`]: crate::Scanner
//! [C]: CScannerExt
//! [Rust]: RustScannerExt

mod c;
mod rust;

pub use self::c::*;
pub use self::rust::*;
