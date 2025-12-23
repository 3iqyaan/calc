use criterion::{criterion_group, criterion_main, Criterion};
use calc::tokenize::tokenize;
use calc::compute::{eval, to_rpn};

fn bench_to_rpn(c: &mut Criterion) {
    let expr = "2 ^ 3 % 3";
    let tokens = tokenize(expr).unwrap();

    c.bench_function("to_rpn", |b| {
        b.iter(|| {
            let _ = to_rpn(tokens.clone());
        })
    });
}

fn bench_eval(c: &mut Criterion) {
    let expr = "2 + 3";
    let tokens = tokenize(expr).unwrap();
    let tokens = to_rpn(tokens);

    c.bench_function("calc", |b| {
        b.iter(|| {
            let _ = eval(tokens.clone()).unwrap();
        })
    });

    c.bench_function("rust", |b| {
        b.iter(|| {
            // Direct Rust calculation
            let _ = 2.0 + 3.0;
        })
    });
}

fn bench_tokenization(c: &mut Criterion) {
    let expr = "2 ^ 3 % 3";
    c.bench_function("tokenize", |b| {
        b.iter(|| {
            let _ = tokenize(expr).unwrap();
        })
    });
}

criterion_group!(benches, bench_tokenization, bench_eval, bench_to_rpn);
criterion_main!(benches);