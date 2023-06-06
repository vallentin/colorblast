use std::fmt;

#[derive(Clone, Debug)]
pub struct AnsiStyle {
    pub fg: Option<AnsiColor>,
    pub bg: Option<AnsiColor>,
}

impl AnsiStyle {
    pub const DEFAULT: Self = Self { fg: None, bg: None };
}

impl Default for AnsiStyle {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl fmt::Display for AnsiStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let codes = [self.fg.map(AnsiCode::Fg), self.bg.map(AnsiCode::Bg)];
        let mut codes = codes.iter().flatten();

        match codes.next() {
            Some(code) => {
                write!(f, "\x1b[")?;
                code.write_code(f)?;
            }
            None => return Ok(()),
        }

        for code in codes {
            write!(f, ";")?;
            code.write_code(f)?;
        }

        write!(f, "m")?;

        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AnsiCode {
    Reset,
    Fg(AnsiColor),
    Bg(AnsiColor),
}

impl AnsiCode {
    fn write_code(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reset => write!(f, "0"),
            Self::Fg(c) => c.write_fg_code(f),
            Self::Bg(c) => c.write_bg_code(f),
        }
    }
}

impl fmt::Display for AnsiCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[")?;
        self.write_code(f)?;
        write!(f, "m")?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AnsiColor(pub [u8; 3]);

impl AnsiColor {
    fn write_fg_code(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self([r, g, b]) = self;
        write!(f, "38;2;{r};{g};{b}")
    }

    fn write_bg_code(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self([r, g, b]) = self;
        write!(f, "48;2;{r};{g};{b}")
    }
}
