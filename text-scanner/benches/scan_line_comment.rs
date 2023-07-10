use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use text_scanner::{Scanner, ScannerResult};

#[inline]
fn scan_line_comment_old<'text>(scanner: &mut Scanner<'text>) -> ScannerResult<'text, &'text str> {
    scanner.scan_with(|scanner| {
        scanner.accept_str("//")?;
        // Note: This does not handle the case where `\r` is not
        // immediately followed by `\n`. However, the test data
        // does not include that case
        scanner.skip_until_char_any(&['\n', '\r']);
        Ok(())
    })
}

#[inline]
fn scan_line_comment_new<'text>(scanner: &mut Scanner<'text>) -> ScannerResult<'text, &'text str> {
    scanner.scan_with(|scanner| {
        scanner.test_str("//")?;
        _ = scanner.next_line();
        Ok(())
    })
}

#[inline]
fn scan<'text, F>(scanner: &mut Scanner<'text>, mut f: F)
where
    F: FnMut(&mut Scanner<'text>) -> ScannerResult<'text, &'text str>,
{
    while scanner.has_remaining_text() {
        let (r, _s) = scanner.skip_whitespace();
        if !r.is_empty() {
            continue;
        }

        if let Ok(_) = f(scanner) {
            continue;
        }

        if let Ok(_) = scanner.next_line_terminator() {
            continue;
        }

        unreachable!();
    }
}

fn bench_accept_vs_test(c: &mut Criterion) {
    let text = r#"
// Hello World
// Hello World
// Hello World
// Hello World
// Hello World
// Hello World
// Hello World
// Hello World
// Hello World
// Hello World
"#;
    let scanner = Scanner::new(black_box(text));

    let mut group = c.benchmark_group("comment");
    group.bench_function("old", |b| {
        b.iter(|| scan(&mut scanner.clone(), scan_line_comment_old));
    });
    group.bench_function("new", |b| {
        b.iter(|| scan(&mut scanner.clone(), scan_line_comment_new));
    });
    group.finish();
}

criterion_group!(benches, bench_accept_vs_test);
criterion_main!(benches);
