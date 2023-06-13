use colorblast::prelude::*;

fn main() {
    example_with_color();
    example_with_option_color();
    example_with_style();
    example_with_option_style();
    println!();

    let r = Style::new().fg([255, 0, 0]); // red
    let g = Style::new().fg([0, 255, 0]); // green
    let b = Style::new().fg([0, 0, 255]); // blue

    let y = Style::new().fg([255, 255, 0]); // yellow
    let c = Style::new().fg([0, 255, 255]); // cyan
    let p = Style::new().fg([255, 0, 255]); // purple

    let w_on_r = r.clone().invert().fg(Color::WHITE); // white on red
    let w_on_g = g.clone().invert().fg(Color::WHITE); // white on green
    let w_on_b = b.clone().invert().fg(Color::WHITE); // white on blue

    let w_on_y = y.clone().invert().fg(Color::WHITE); // white on yellow
    let w_on_c = c.clone().invert().fg(Color::WHITE); // white on cyan
    let w_on_p = p.clone().invert().fg(Color::WHITE); // white on purple

    let tokens = vec![
        (Some(r), "red"),
        (None, " "),
        (Some(g), "green"),
        (None, " "),
        (Some(b), "blue"),
        (None, "\n"),
        (Some(y), "yellow"),
        (None, " "),
        (Some(c), "cyan"),
        (None, " "),
        (Some(p), "purple"),
        (None, "\n"),
        (Some(w_on_r), "red"),
        (None, " "),
        (Some(w_on_g), "green"),
        (None, " "),
        (Some(w_on_b), "blue"),
        (None, "\n"),
        (Some(w_on_y), "yellow"),
        (None, " "),
        (Some(w_on_c), "cyan"),
        (None, " "),
        (Some(w_on_p), "purple"),
        (None, "\n"),
    ];
    // Print styled tokens to the standard output
    println_styled_tokens(tokens.clone());

    // Render the styled tokens to HTML
    let html = render_html(tokens);
    println!("{}", html);
}

fn example_with_color() {
    // Specify a color for each token
    let tokens = vec![
        ([0xFF, 0x00, 0x00], "red"),
        ([0xFF, 0xFF, 0xFF], " "),
        ([0x00, 0xFF, 0x00], "green"),
        ([0xFF, 0xFF, 0xFF], " "),
        ([0x00, 0x00, 0xFF], "blue"),
    ];
    println_styled_tokens(tokens);
}

fn example_with_option_color() {
    // Wrap in `Option` instead of specifying a color
    // for tokens where none is needed
    let tokens = vec![
        (Some([0xFF, 0x00, 0x00]), "red"),
        (None, " "),
        (Some([0x00, 0xFF, 0x00]), "green"),
        (None, " "),
        (Some([0x00, 0x00, 0xFF]), "blue"),
    ];
    println_styled_tokens(tokens);
}

fn example_with_style() {
    // Use `Style`s for more control
    let w_on_r = Style::new().fg(Color::WHITE).bg([0xFF, 0x00, 0x00]); // white on red
    let w_on_g = Style::new().fg(Color::WHITE).bg([0x00, 0xFF, 0x00]); // white on green
    let w_on_b = Style::new().fg(Color::WHITE).bg([0x00, 0x00, 0xFF]); // white on blue

    let tokens = vec![
        (w_on_r, "red"),
        (Style::NONE, " "),
        (w_on_g, "green"),
        (Style::NONE, " "),
        (w_on_b, "blue"),
    ];
    println_styled_tokens(tokens);
}

fn example_with_option_style() {
    // Use `Style`s for more control
    let w_on_r = Style::new().fg(Color::WHITE).bg([0xFF, 0x00, 0x00]); // white on red
    let w_on_g = Style::new().fg(Color::WHITE).bg([0x00, 0xFF, 0x00]); // white on green
    let w_on_b = Style::new().fg(Color::WHITE).bg([0x00, 0x00, 0xFF]); // white on blue

    // Wrap in `Option` instead of using `Style::NONE`
    let tokens = vec![
        (Some(w_on_r), "red"),
        (None, " "),
        (Some(w_on_g), "green"),
        (None, " "),
        (Some(w_on_b), "blue"),
    ];
    println_styled_tokens(tokens);
}
