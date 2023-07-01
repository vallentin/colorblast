# text-scanner

[![CI](https://github.com/vallentin/colorblast/actions/workflows/ci-text-scanner.yml/badge.svg)](https://github.com/vallentin/colorblast/actions/workflows/ci-text-scanner.yml)
[![Latest Version](https://img.shields.io/crates/v/text-scanner.svg)](https://crates.io/crates/text-scanner)
[![Docs](https://docs.rs/text-scanner/badge.svg)](https://docs.rs/text-scanner)
[![License](https://img.shields.io/github/license/vallentin/colorblast.svg)](https://github.com/vallentin/colorblast)

**Warning:** This library is experimental and may change drastically in `0.0.*` versions.

A UTF-8 char-oriented, zero-copy, text and code scanning library.

This crate implements a UTF-8 [`char`]-based text [`Scanner`], which includes
various methods for scanning a string slice, as well as backtracking capabilities,
which can be used to implement lexers for tokenizing text or code.

[`char`]: https://doc.rust-lang.org/std/primitive.char.html
[`Scanner`]: https://docs.rs/text-scanner/*/text_scanner/struct.Scanner.html

Scanning extensions for existing languages and formats have already been
implemented, such as for [Rust][RustExt], [C][CExt], [Python][PythonExt],
[CSS][CSSExt], [SCSS][SCSSExt], [JSON][JSONExt], [JSON with Comments][JSONCExt],
_[and many more][text-scanner-ext]_.

[RustExt]: https://docs.rs/text-scanner/*/text_scanner/ext/trait.RustScannerExt.html
[CExt]: https://docs.rs/text-scanner/*/text_scanner/ext/trait.CScannerExt.html
[PythonExt]: https://docs.rs/text-scanner/*/text_scanner/ext/trait.PythonScannerExt.html
[CSSExt]: https://docs.rs/text-scanner/*/text_scanner/ext/trait.CssScannerExt.html
[SCSSExt]: https://docs.rs/text-scanner/*/text_scanner/ext/trait.ScssScannerExt.html
[JSONExt]: https://docs.rs/text-scanner/*/text_scanner/ext/trait.JsonScannerExt.html
[JSONCExt]: https://docs.rs/text-scanner/*/text_scanner/ext/trait.JsonCScannerExt.html
[text-scanner-ext]: https://docs.rs/text-scanner/*/text_scanner/ext/index.html

For examples of lexers implemented using [`Scanner`], see the [`any-lexer` crate],
which implements lexers for e.g. [Rust][RustLexer], [C][CLexer], [Python][PythonLexer],
[CSS][CSSLexer], [SCSS][SCSSLexer], [JSON][JSONLexer], [JSON with Comments][JSONCLexer],
_[and many more][lexers]_.

[`any-lexer` crate]: https://crates.io/crates/any-lexer
[RustLexer]: https://docs.rs/any-lexer/*/any_lexer/struct.RustLexer.html
[CLexer]: https://docs.rs/any-lexer/*/any_lexer/struct.CLexer.html
[PythonLexer]: https://docs.rs/any-lexer/*/any_lexer/struct.PythonLexer.html
[CSSLexer]: https://docs.rs/any-lexer/*/any_lexer/struct.CssLexer.html
[SCSSLexer]: https://docs.rs/any-lexer/*/any_lexer/struct.ScssLexer.html
[JSONLexer]: https://docs.rs/any-lexer/*/any_lexer/struct.JsonLexer.html
[JSONCLexer]: https://docs.rs/any-lexer/*/any_lexer/struct.JsonCLexer.html
[lexers]: https://docs.rs/any-lexer/*/any_lexer/
