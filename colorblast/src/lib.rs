#![forbid(unsafe_code)]
#![forbid(elided_lifetimes_in_paths)]

pub mod prelude {
    pub use super::style::prelude::*;

    pub use super::{print_stylized_tokens, println_stylized_tokens};
}

mod ansi;
mod style;

pub use crate::style::*;

use std::fmt;

use crate::ansi::{AnsiCode, AnsiColor, AnsiStyle};

/// Stylizes all `tokens` and prints them to the standard output.
pub fn print_stylized_tokens<Sty, Tok, I>(tokens: I)
where
    I: IntoIterator<Item = (Sty, Tok)>,
    Sty: AsStyle,
    Tok: fmt::Display,
{
    use AnsiCode::Reset;
    for (sty, tok) in tokens {
        let style = match sty.as_style() {
            Some(style) => style,
            None => {
                print!("{tok}");
                continue;
            }
        };

        let fg = style.fg.map(|Color([r, g, b, _a])| AnsiColor([r, g, b]));
        let bg = style.bg.map(|Color([r, g, b, _a])| AnsiColor([r, g, b]));
        let style = AnsiStyle { fg, bg };

        print!("{style}{tok}{Reset}");
    }
}

/// Same as [`print_stylized_tokens()`] + `println!()` after.
pub fn println_stylized_tokens<Sty, Tok, I>(tokens: I)
where
    I: IntoIterator<Item = (Sty, Tok)>,
    Sty: AsStyle,
    Tok: fmt::Display,
{
    print_stylized_tokens(tokens);
    println!();
}
