#![forbid(unsafe_code)]
#![forbid(elided_lifetimes_in_paths)]

use std::error;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::exit;

use colorblast::{Color, Lexer, StylizeToken};
use image::{Pixel, Rgba, RgbaImage};
use rusttype::{point, Font, GlyphId, Point, PositionedGlyph, Rect, Scale};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(parse(from_os_str))]
    font_file: PathBuf,
    #[structopt(parse(from_os_str))]
    file: PathBuf,
    #[structopt(short, long, parse(from_os_str), default_value = "code.png")]
    output: PathBuf,
    #[structopt(long)]
    font_size: Option<f32>,
    #[structopt(long)]
    margin: Option<u32>,
}

fn main() {
    exit({
        let opt = Opt::from_args();
        let code = match try_main(&opt) {
            Ok(()) => 0,
            Err(err) => {
                eprintln!("error: {err}");
                1
            }
        };
        let _ = io::stdout().flush();
        let _ = io::stderr().flush();
        code
    });
}

fn try_main(opt: &Opt) -> Result<(), Box<dyn error::Error>> {
    println!("Loading code `{}`", opt.file.display());
    let code = fs::read_to_string(&opt.file)?;

    if code.is_empty() {
        println!("`{}` is empty", opt.file.display());
        println!("no image rendered");
        return Ok(());
    }

    println!("Loading font `{}`", opt.font_file.display());
    let font_data = fs::read(&opt.font_file)?;
    let font = Font::try_from_bytes(&font_data).expect("invalid font data");

    let font_size = opt.font_size.unwrap_or(12.0);
    let scale = Scale::uniform(font_size);

    let margin = opt.margin.unwrap_or(20);
    let start = point(margin as f32, margin as f32);

    let mut layout = Layout::new(&font, start, scale);

    let (w, h) = {
        println!("Measuring...");

        let mut layout = layout.clone();
        let rect = code.chars().map(|c| layout.next_glyph(c)).fold(
            Rect {
                min: point(i32::MAX, i32::MAX),
                max: point(i32::MIN, i32::MIN),
            },
            |mut rect, glyph| {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    rect.min.x = rect.min.x.min(bounding_box.min.x);
                    rect.min.y = rect.min.y.min(bounding_box.min.y);
                    rect.max.x = rect.max.x.max(bounding_box.max.x);
                    rect.max.y = rect.max.y.max(bounding_box.max.y);
                }
                rect
            },
        );

        let w = rect.max.x - rect.min.x;
        let h = rect.max.y - rect.min.y;

        let v_metrics = font.v_metrics(scale);
        let h = h + (v_metrics.descent.abs().ceil() as i32);

        let w = w + ((margin * 2) as i32);
        let h = h + ((margin * 2) as i32);

        (w as u32, h as u32)
    };

    println!("Rendering...");
    let (bg_r, bg_g, bg_b) = (30, 30, 30);
    let mut img = RgbaImage::from_pixel(w, h, Rgba([bg_r, bg_g, bg_b, 255]));

    let lexer = Lexer::Rust.into_lexer(&code);
    for (tok, span) in lexer {
        let style = tok.style(&span);

        let (r, g, b) = match style.fg {
            Some(Color([r, g, b, _a])) => (r, g, b),
            _ => (255, 255, 255),
        };

        for c in span.as_str().chars() {
            let glyph = layout.next_glyph(c);

            if c.is_control() {
                continue;
            }

            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let x = x.wrapping_add_signed(bounding_box.min.x);
                    let y = y.wrapping_add_signed(bounding_box.min.y);

                    if let Some(pixel) = img.get_pixel_mut_checked(x, y) {
                        pixel.blend(&Rgba([r, g, b, (v * 255.0) as u8]));
                    }
                });
            }
        }
    }

    img.save(&opt.output)?;
    println!("Rendered `{}`", opt.output.display());

    Ok(())
}

#[derive(Clone)]
pub struct Layout<'a, 'font> {
    font: &'a Font<'font>,
    scale: Scale,
    start: Point<f32>,
    caret: Point<f32>,
    advance_height: f32,
    last_glyph_id: Option<GlyphId>,
}

impl<'a, 'font> Layout<'a, 'font> {
    pub fn new(font: &'a Font<'font>, start: Point<f32>, scale: Scale) -> Self {
        let v_metrics = font.v_metrics(scale);
        let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
        Self {
            font,
            scale,
            start,
            caret: point(start.x, start.y + v_metrics.ascent),
            advance_height,
            last_glyph_id: None,
        }
    }

    pub fn next_glyph(&mut self, c: char) -> PositionedGlyph<'font> {
        let base_glyph = self.font.glyph(c);

        if let Some(last_glyph_id) = self.last_glyph_id.take() {
            self.caret.x += self
                .font
                .pair_kerning(self.scale, last_glyph_id, base_glyph.id());
        }

        self.last_glyph_id = Some(base_glyph.id());

        let glyph = base_glyph.scaled(self.scale).positioned(self.caret);

        if c == '\n' {
            self.caret.x = self.start.x;
            self.caret.y += self.advance_height;
        } else {
            self.caret.x += glyph.unpositioned().h_metrics().advance_width;
        }

        glyph
    }
}
