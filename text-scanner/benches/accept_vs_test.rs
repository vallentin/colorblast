use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use text_scanner::{Scanner, ScannerItem};

fn bench_accept_vs_test(c: &mut Criterion) {
    let scanner = Scanner::new(black_box("// Hello World"));

    let mut group = c.benchmark_group("str");
    group.bench_function("accept_str", |b| {
        b.iter(|| -> ScannerItem<&'_ str> {
            let mut scanner = scanner.clone();
            scanner.accept_str(black_box("//")).unwrap()
        });
    });
    group.bench_function("test_str", |b| {
        b.iter(|| -> ScannerItem<&'_ str> {
            let mut scanner = scanner.clone();
            scanner.test_str(black_box("//")).unwrap()
        });
    });
    group.finish();

    let mut group = c.benchmark_group("str_any");
    group.bench_function("accept_str_any", |b| {
        b.iter(|| -> ScannerItem<&'_ str> {
            let mut scanner = scanner.clone();
            scanner
                .accept_str_any(&black_box(["// foo", "// bar", "// baz", "//"]))
                .unwrap()
        });
    });
    group.bench_function("test_str_any", |b| {
        b.iter(|| -> ScannerItem<&'_ str> {
            let mut scanner = scanner.clone();
            scanner
                .test_str_any(&black_box(["// foo", "// bar", "// baz", "//"]))
                .unwrap()
        });
    });
    group.finish();
}

criterion_group!(benches, bench_accept_vs_test);
criterion_main!(benches);
