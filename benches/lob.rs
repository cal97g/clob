use criterion::{black_box, criterion_group, criterion_main, Criterion};
use clob::orderbook::{Orderbook};
use rust_decimal::prelude::*;

fn instantiate(ask: Decimal, bid: Decimal, min_step: Decimal) {
  Orderbook::new(bid, ask, min_step);
}

fn criterion_benchmark(c: &mut Criterion) {
    let ask: Decimal = Decimal::from_f32(100.0).unwrap();
    let bid: Decimal = Decimal::from_f32(99.0).unwrap();
    let min_step: Decimal = Decimal::from_f32(0.10).unwrap();

    c.bench_function("instantiate", |b| b.iter(|| instantiate(ask, bid, min_step)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);