use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use text_scanner::{Scanner, ScannerItem, ScannerResult};

trait ScannerFuncs<'text> {
    fn accept_if<F>(&mut self, f: F) -> ScannerResult<'text, char>
    where
        F: FnOnce(char) -> bool;

    #[inline]
    fn accept_char(&mut self, expected: char) -> ScannerResult<'text, char> {
        self.accept_if(|c| c == expected)
    }

    #[inline]
    fn accept_char_any(&mut self, expected: &[char]) -> ScannerResult<'text, char> {
        self.accept_if(|c| expected.contains(&c))
    }
}

impl<'text> ScannerFuncs<'text> for Scanner<'text> {
    #[inline]
    fn accept_if<F>(&mut self, f: F) -> ScannerResult<'text, char>
    where
        F: FnOnce(char) -> bool,
    {
        Scanner::accept_if(self, f)
    }

    #[inline]
    fn accept_char(&mut self, expected: char) -> ScannerResult<'text, char> {
        Scanner::accept_char(self, expected)
    }

    #[inline]
    fn accept_char_any(&mut self, expected: &[char]) -> ScannerResult<'text, char> {
        Scanner::accept_char_any(self, expected)
    }
}

#[repr(transparent)]
struct ScannerPeek<'text>(Scanner<'text>);

impl<'text> ScannerFuncs<'text> for ScannerPeek<'text> {
    #[inline]
    fn accept_if<F>(&mut self, f: F) -> ScannerResult<'text, char>
    where
        F: FnOnce(char) -> bool,
    {
        let (r, c) = self.0.peek()?;
        if f(c) {
            self.0.set_cursor_pos(r.end);
            Ok((r, c))
        } else {
            let cursor = self.0.cursor_pos();
            Err((cursor..cursor, ""))
        }
    }
}

#[repr(transparent)]
struct ScannerPattern<'text>(Scanner<'text>);

impl<'text> ScannerPattern<'text> {
    #[inline]
    fn test<T, F>(&mut self, f: F) -> ScannerResult<'text, T>
    where
        F: FnOnce(&Scanner<'text>) -> Option<(usize, T)>,
    {
        match f(&self.0) {
            Some((len_utf8, c)) => {
                let start = self.0.cursor_pos();
                let end = start + len_utf8;
                self.0.set_cursor_pos(end);
                Ok((start..end, c))
            }
            None => {
                let cursor = self.0.cursor_pos();
                Err((cursor..cursor, ""))
            }
        }
    }
}

impl<'text> ScannerFuncs<'text> for ScannerPattern<'text> {
    #[inline]
    fn accept_if<F>(&mut self, f: F) -> ScannerResult<'text, char>
    where
        F: FnOnce(char) -> bool,
    {
        self.test(|scanner| {
            let c = scanner.remaining_text().chars().next();
            match c {
                Some(c) if f(c) => Some((c.len_utf8(), c)),
                _ => None,
            }
        })
    }

    #[inline]
    fn accept_char(&mut self, expected: char) -> ScannerResult<'text, char> {
        self.test(|scanner| {
            scanner
                .remaining_text()
                .starts_with(expected)
                .then(|| (expected.len_utf8(), expected))
        })
    }
}

#[inline]
fn test_accept_if<'text, S: ScannerFuncs<'text>>(scanner: &mut S) -> ScannerItem<char> {
    scanner
        .accept_if(black_box(|c: char| c.is_alphanumeric() || (c == '/')))
        .unwrap()
}

#[inline]
fn test_accept_char<'text, S: ScannerFuncs<'text>>(
    scanner: &mut S,
) -> (ScannerItem<char>, ScannerItem<char>) {
    (
        scanner.accept_char(black_box('/')).unwrap(),
        scanner.accept_char(black_box('/')).unwrap(),
    )
}

#[inline]
fn test_accept_char_any<'text, S: ScannerFuncs<'text>>(
    scanner: &mut S,
) -> (ScannerItem<char>, ScannerItem<char>) {
    (
        scanner
            .accept_char_any(&black_box(['f', 'o', 'o', 'b', 'a', 'r', '/']))
            .unwrap(),
        scanner
            .accept_char_any(&black_box(['f', 'o', 'o', 'b', 'a', 'r', '/']))
            .unwrap(),
    )
}

fn bench_accept_impl(c: &mut Criterion) {
    let scanner = Scanner::new(black_box("// Hello World"));

    let mut group = c.benchmark_group("accept_if");
    group.bench_function("peek", |b| {
        b.iter(|| test_accept_if(&mut ScannerPeek(scanner.clone())));
    });
    group.bench_function("pattern", |b| {
        b.iter(|| test_accept_if(&mut ScannerPattern(scanner.clone())));
    });
    group.bench_function("current", |b| {
        b.iter(|| test_accept_if(&mut scanner.clone()));
    });
    group.finish();

    let mut group = c.benchmark_group("accept_char");
    group.bench_function("peek", |b| {
        b.iter(|| test_accept_char(&mut ScannerPeek(scanner.clone())));
    });
    group.bench_function("pattern", |b| {
        b.iter(|| test_accept_char(&mut ScannerPattern(scanner.clone())));
    });
    group.bench_function("current", |b| {
        b.iter(|| test_accept_char(&mut scanner.clone()));
    });
    group.finish();

    let mut group = c.benchmark_group("accept_char_any");
    group.bench_function("peek", |b| {
        b.iter(|| test_accept_char_any(&mut ScannerPeek(scanner.clone())));
    });
    group.bench_function("pattern", |b| {
        b.iter(|| test_accept_char_any(&mut ScannerPattern(scanner.clone())));
    });
    group.bench_function("current", |b| {
        b.iter(|| test_accept_char_any(&mut scanner.clone()));
    });
    group.finish();
}

criterion_group!(benches, bench_accept_impl);
criterion_main!(benches);
