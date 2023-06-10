use colorblast::lexers::RustLexer;
use colorblast::prelude::*;

fn main() {
    let code = include_str!("rust.rs");
    let lexer = RustLexer::new(code);
    println_stylized_tokens(lexer);
}
