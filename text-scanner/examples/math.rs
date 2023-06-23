use std::env;
use std::error;
use std::io::{self, Read, Write};

use text_scanner::Scanner;

#[derive(PartialEq, Clone, Debug)]
enum Token<'text> {
    Ident(&'text str),
    Int(i32),
    Float(f32),
    Sym(Sym),
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Sym {
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

impl<'text> Token<'text> {
    fn parse_token(scanner: &mut Scanner<'text>) -> Result<Option<Self>, Box<dyn error::Error>> {
        scanner.skip_whitespace();

        if let Ok((first, _c)) = scanner.accept_if(|c| c.is_alphabetic() || (c == '_')) {
            let (last, _s) = scanner.skip_while(|c| c.is_alphanumeric() || (c == '_'));
            return Ok(Some(Self::Ident(&scanner.text()[first.start..last.end])));
        }

        if let Ok((first, _c)) = scanner.accept_if(|c| c.is_ascii_digit()) {
            let (last, _s) = scanner.skip_while(|c| c.is_ascii_digit());

            if scanner.accept_char('.').is_ok() {
                let (last, _s) = scanner.skip_while(|c| c.is_ascii_digit());
                let text = &scanner.text()[first.start..last.end];
                let f = text.parse()?;
                return Ok(Some(Self::Float(f)));
            } else {
                let text = &scanner.text()[first.start..last.end];
                let f = text.parse()?;
                return Ok(Some(Self::Int(f)));
            }
        }

        if let Some(sym) = Sym::parse_token(scanner) {
            return Ok(Some(Self::Sym(sym)));
        }

        Ok(None)
    }
}

impl<'text> Sym {
    fn parse_token(scanner: &mut Scanner<'text>) -> Option<Self> {
        let (_r, c) = scanner
            .accept_char_any(&['+', '-', '*', '/', '(', ')'])
            .ok()?;
        match c {
            '+' => Some(Self::Plus),
            '-' => Some(Self::Minus),
            '*' => Some(Self::Star),
            '/' => Some(Self::Slash),
            '(' => Some(Self::LParen),
            ')' => Some(Self::RParen),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Expr<'text> {
    Ident(&'text str),
    Int(i32),
    Float(f32),
    Add(Box<Self>, Box<Self>),
    Sub(Box<Self>, Box<Self>),
    Mul(Box<Self>, Box<Self>),
    Div(Box<Self>, Box<Self>),
    Pos(Box<Self>),
    Neg(Box<Self>),
}

impl<'text> Expr<'text> {
    fn parse(text: &'text str) -> Result<Self, Box<dyn error::Error>> {
        ExprParser::new(text).parse()
    }
}

struct ExprParser<'text> {
    scanner: Scanner<'text>,
    next: Option<Token<'text>>,
}

impl<'text> ExprParser<'text> {
    fn new(text: &'text str) -> Self {
        Self {
            scanner: Scanner::new(text),
            next: None,
        }
    }

    fn next_token(&mut self) -> Result<Option<Token<'text>>, Box<dyn error::Error>> {
        if let Some(tok) = self.next.take() {
            return Ok(Some(tok));
        }
        Token::parse_token(&mut self.scanner)
    }

    fn peek_token(&mut self) -> Result<Option<&Token<'text>>, Box<dyn error::Error>> {
        if self.next.is_none() {
            self.next = self.next_token()?;
        }
        Ok(self.next.as_ref())
    }

    fn parse(&mut self) -> Result<Expr<'text>, Box<dyn error::Error>> {
        let expr = self.parse_expr()?;

        if let Some(tok) = self.next_token()? {
            return Err(format!("expected end of input, received {:?}", tok).into());
        }

        Ok(expr)
    }

    fn parse_expr(&mut self) -> Result<Expr<'text>, Box<dyn error::Error>> {
        self.parse_expr_additive()
    }

    fn parse_expr_additive(&mut self) -> Result<Expr<'text>, Box<dyn error::Error>> {
        let mut expr = self.parse_expr_multiplicative()?;

        while let Some(&Token::Sym(op @ (Sym::Plus | Sym::Minus))) = self.peek_token()? {
            _ = self.next_token(); // Eat `Token::Sym`

            let rhs = self.parse_expr_multiplicative()?;

            expr = match op {
                Sym::Plus => Expr::Add(Box::new(expr), Box::new(rhs)),
                Sym::Minus => Expr::Sub(Box::new(expr), Box::new(rhs)),
                _ => unreachable!(),
            };
        }

        Ok(expr)
    }

    fn parse_expr_multiplicative(&mut self) -> Result<Expr<'text>, Box<dyn error::Error>> {
        let mut expr = self.parse_expr_unary()?;

        while let Some(&Token::Sym(op @ (Sym::Star | Sym::Slash))) = self.peek_token()? {
            _ = self.next_token(); // Eat `Token::Sym`

            let rhs = self.parse_expr_multiplicative()?;

            expr = match op {
                Sym::Star => Expr::Mul(Box::new(expr), Box::new(rhs)),
                Sym::Slash => Expr::Div(Box::new(expr), Box::new(rhs)),
                _ => unreachable!(),
            };
        }

        Ok(expr)
    }

    fn parse_expr_unary(&mut self) -> Result<Expr<'text>, Box<dyn error::Error>> {
        if let Some(&Token::Sym(op @ (Sym::Plus | Sym::Minus))) = self.peek_token()? {
            _ = self.next_token(); // Eat `Token::Sym`

            let expr = self.parse_expr_unary()?;

            match op {
                Sym::Plus => Ok(Expr::Pos(Box::new(expr))),
                Sym::Minus => Ok(Expr::Neg(Box::new(expr))),
                _ => unreachable!(),
            }
        } else {
            self.parse_expr_value()
        }
    }

    fn parse_expr_value(&mut self) -> Result<Expr<'text>, Box<dyn error::Error>> {
        let tok = self
            .next_token()?
            .ok_or_else(|| "unexpected end of input")?;
        match tok {
            Token::Ident(ident) => Ok(Expr::Ident(ident)),
            Token::Int(i) => Ok(Expr::Int(i)),
            Token::Float(f) => Ok(Expr::Float(f)),
            Token::Sym(Sym::LParen) => {
                let expr = self.parse_expr()?;

                let tok = self
                    .next_token()?
                    .ok_or_else(|| "unexpected end of input")?;
                if tok != Token::Sym(Sym::RParen) {
                    return Err(format!("expected `)` found {:?}", tok).into());
                }

                Ok(expr)
            }
            _ => Err(format!("unexpected token {:?}", tok).into()),
        }
    }
}

impl<'text> Expr<'text> {
    fn eval(&self) -> Result<f32, Box<dyn error::Error>> {
        match self {
            &Self::Ident(ident) => match ident {
                "pi" | "PI" => Ok(std::f32::consts::PI),
                "tau" | "TAU" => Ok(std::f32::consts::TAU),
                _ => Err(format!("unknown ident `{}`, expected `pi` or `tau`", ident).into()),
            },
            &Self::Int(i) => Ok(i as f32),
            &Self::Float(f) => Ok(f),
            Self::Add(lhs, rhs) => Ok(lhs.eval()? + rhs.eval()?),
            Self::Sub(lhs, rhs) => Ok(lhs.eval()? - rhs.eval()?),
            Self::Mul(lhs, rhs) => Ok(lhs.eval()? * rhs.eval()?),
            Self::Div(lhs, rhs) => Ok(lhs.eval()? / rhs.eval()?),
            Self::Pos(operand) => Ok(operand.eval()?),
            Self::Neg(operand) => Ok(-operand.eval()?),
        }
    }
}

fn main() {
    eval_print_input("2 + 3");
    eval_print_input("-(2 + 3)");
    eval_print_input("-(2 + 3) * -5");
    eval_print_input("3 * pi - tau");

    let mut read_stdin = false;
    let mut repl = false;

    for arg in env::args().skip(1) {
        if arg == "-i" {
            repl = true;
        } else if arg == "-" {
            read_stdin = true;
        } else {
            eval_print_input(&arg);
        }
    }

    if read_stdin {
        println!();

        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).unwrap();
        eval_print_input(&buf);
    } else if repl {
        println!();
        println!("Enter `exit` or press Ctrl+C to exit repl");

        let mut input = String::new();

        loop {
            print!("> ");
            _ = io::stdout().flush();
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input.is_empty() {
                continue;
            } else if input == "exit" {
                break;
            }

            eval(&input);
        }
    }
}

fn eval_print_input(expr: &str) {
    println!("{}", expr);
    eval(expr);
}

fn eval(expr: &str) {
    match Expr::parse(expr) {
        Ok(expr) => match expr.eval() {
            Ok(val) => println!("= {}", val),
            Err(err) => eprintln!("eval error: {}", err),
        },
        Err(err) => eprintln!("parse error: {}", err),
    }
}
