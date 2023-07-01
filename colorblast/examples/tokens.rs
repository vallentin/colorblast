use colorblast::{prelude::*, StylizeToken};

fn main() {
    let tokens = Token::VARIANTS
        .iter()
        .copied()
        .map(|tok| (tok, format!("Token::{:?}\n", tok)))
        .collect::<Vec<_>>();

    println_styled_tokens(tokens.iter().map(|(tok, span)| {
        let span = TokenSpan::new(span, 0..span.len());
        let sty = tok.style(&span);
        (sty, span)
    }));
}
