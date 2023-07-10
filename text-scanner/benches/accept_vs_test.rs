use criterion::{criterion_group, criterion_main, Criterion};
use text_scanner::{Scanner, ScannerItem};

fn bench_accept_vs_test(c: &mut Criterion) {
    let scanner = Scanner::new("// Hello World");

    let mut group = c.benchmark_group("str");
    group.bench_function("accept_str", |b| {
        b.iter(|| -> ScannerItem<&'_ str> {
            let mut scanner = scanner.clone();
            scanner.accept_str("//").unwrap()
        });
    });
    group.bench_function("test_str", |b| {
        b.iter(|| -> ScannerItem<&'_ str> {
            let mut scanner = scanner.clone();
            scanner.test_str("//").unwrap()
        });
    });
    group.finish();

    let mut group = c.benchmark_group("str_any");
    group.bench_function("accept_str_any", |b| {
        b.iter(|| -> ScannerItem<&'_ str> {
            let mut scanner = scanner.clone();
            scanner
                .accept_str_any(&["// foo", "// bar", "// baz", "//"])
                .unwrap()
        });
    });
    group.bench_function("test_str_any", |b| {
        b.iter(|| -> ScannerItem<&'_ str> {
            let mut scanner = scanner.clone();
            scanner
                .test_str_any(&["// foo", "// bar", "// baz", "//"])
                .unwrap()
        });
    });
    group.finish();
}

criterion_group!(benches, bench_accept_vs_test);
criterion_main!(benches);
