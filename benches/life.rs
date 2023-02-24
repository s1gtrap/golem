use criterion::{criterion_group, criterion_main, Criterion};

use golly::Universe;

fn uni(c: &mut Criterion) {
    let mut u = Universe::from(
        r#"     
  x  
  x  
  x  
     "#,
    );
    c.bench_function("struct based", |b| b.iter(|| u.step()));
}

fn func(c: &mut Criterion) {
    let (w, h, mut s, mut cs) = golly::fns::from_with_mask(
        r#"     
  x  
  x  
  x  
     "#,
        128,
    );
    c.bench_function("func based", |b| {
        b.iter(|| golly::fns::step(w, h, &mut s, &mut cs, 128))
    });
}

criterion_group!(benches, uni, func);
criterion_main!(benches);
