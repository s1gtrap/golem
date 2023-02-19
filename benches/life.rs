use criterion::{criterion_group, criterion_main, Criterion};

use golly::Universe;

fn criterion_benchmark(c: &mut Criterion) {
    let mut u = Universe::from(
        r#"     
  x  
  x  
  x  
     "#,
    );
    c.bench_function("fib 20", |b| b.iter(|| u.step()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
