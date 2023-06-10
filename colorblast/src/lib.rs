#![forbid(unsafe_code)]
#![forbid(elided_lifetimes_in_paths)]

pub mod lexers;

pub mod prelude {
    pub use super::style::prelude::*;

    pub use super::{print_styled_tokens, println_styled_tokens};
    pub use super::{print_stylized_tokens, println_stylized_tokens};
}

mod ansi;
mod style;
mod stylize;

pub use crate::style::*;
pub use crate::stylize::StylizeToken;

use std::fmt;

use any_lexer::TokenSpan;

use crate::ansi::{AnsiCode, AnsiColor, AnsiStyle};

/// Stylizes all `tokens` and prints them to the standard output.
#[inline]
pub fn print_stylized_tokens<'text, Tok, I>(tokens: I)
where
    I: IntoIterator<Item = (Tok, TokenSpan<'text>)>,
    Tok: StylizeToken,
{
    for (tok, span) in tokens {
        print_styled_token(&span, Some(tok.style(&span)));
    }
}

/// Same as [`print_stylized_tokens()`] + `println!()` after.
#[inline]
pub fn println_stylized_tokens<'text, Tok, I>(tokens: I)
where
    I: IntoIterator<Item = (Tok, TokenSpan<'text>)>,
    Tok: StylizeToken,
{
    print_stylized_tokens(tokens);
    println!();
}

/// Prints all styled `tokens` to the standard output.
#[inline]
pub fn print_styled_tokens<Sty, Tok, I>(tokens: I)
where
    I: IntoIterator<Item = (Sty, Tok)>,
    Sty: AsStyle,
    Tok: fmt::Display,
{
    for (sty, tok) in tokens {
        print_styled_token(tok, sty.as_style());
    }
}

/// Same as [`print_styled_tokens()`] + `println!()` after.
#[inline]
pub fn println_styled_tokens<Sty, Tok, I>(tokens: I)
where
    I: IntoIterator<Item = (Sty, Tok)>,
    Sty: AsStyle,
    Tok: fmt::Display,
{
    print_styled_tokens(tokens);
    println!();
}

#[inline]
fn print_styled_token<Tok>(token: Tok, style: Option<Style>)
where
    Tok: fmt::Display,
{
    use AnsiCode::Reset;

    let style = match style {
        Some(style) => style,
        None => {
            print!("{token}");
            return;
        }
    };

    let fg = style.fg.map(|Color([r, g, b, _a])| AnsiColor([r, g, b]));
    let bg = style.bg.map(|Color([r, g, b, _a])| AnsiColor([r, g, b]));
    let style = AnsiStyle { fg, bg };

    print!("{style}{token}{Reset}");
}
