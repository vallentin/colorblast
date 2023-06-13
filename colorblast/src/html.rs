use std::fmt::Write;
use std::iter;

use crate::style::{AsStyle, Color};

pub fn render_html<Sty, Tok, I>(tokens: I) -> String
where
    I: IntoIterator<Item = (Sty, Tok)>,
    Sty: AsStyle,
    Tok: AsRef<str>,
{
    let mut html = String::new();
    render_html_into(&mut html, tokens);
    html
}

pub fn render_html_into<Sty, Tok, I>(html: &mut String, tokens: I)
where
    I: IntoIterator<Item = (Sty, Tok)>,
    Sty: AsStyle,
    Tok: AsRef<str>,
{
    for (sty, tok) in tokens {
        let style = match sty.as_style() {
            Some(style) => style,
            None => {
                for part in escape_html(tok.as_ref()) {
                    html.push_str(part);
                }
                continue;
            }
        };

        html.push_str("<span style=\"");

        let style = style
            .fg
            .map(|col| ("color:", col))
            .into_iter()
            .chain(style.bg.map(|col| ("background-color:", col)));

        for (i, (name, col)) in style.enumerate() {
            if i > 0 {
                html.push(';');
            }

            html.push_str(name);

            let Color([r, g, b, a]) = col;
            if a == 255 {
                html.push('#');
                html.extend(u8_to_hex(r));
                html.extend(u8_to_hex(g));
                html.extend(u8_to_hex(b));
            } else {
                html.push_str("rgb(");
                // TODO: unwrap
                write!(html, "{r},{g},{b},{}", (a as f32) / 255.0).unwrap();
                html.push(')');
            }
        }

        html.push_str("\">");

        for part in escape_html(tok.as_ref()) {
            html.push_str(part);
        }

        html.push_str("</span>");
    }
}

fn escape_html(mut text: &str) -> impl Iterator<Item = &str> {
    let mut next = None;
    iter::from_fn(move || {
        if next.is_some() {
            return next.take();
        }

        let index = text.find(['<', '>', '&', '"', '\'']);
        match index {
            Some(i) => {
                let until = &text[..i];
                next = Some(match &text[i..(i + 1)] {
                    "<" => "&lt;",
                    ">" => "&gt;",
                    "&" => "&amp;",
                    "\"" => "&quot;",
                    "'" => "&#x27;",
                    _ => unreachable!(),
                });
                text = &text[(i + 1)..];
                Some(until)
            }
            None if text.is_empty() => None,
            None => {
                let rest = text;
                text = &text[text.len()..];
                Some(rest)
            }
        }
    })
}

fn u8_to_hex(i: u8) -> [char; 2] {
    let high = u8_low_to_hex(i >> 4) as char;
    let low = u8_low_to_hex(i) as char;
    [high, low]
}

fn u8_low_to_hex(i: u8) -> u8 {
    let low = i & 0x0F;
    if low > 9 {
        b'A' + (low - 10)
    } else {
        b'0' + low
    }
}
