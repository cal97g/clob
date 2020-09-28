use criterion::{black_box, criterion_group, criterion_main, Criterion};
use clob::orderbook::{Orderbook};
use rust_decimal::prelude::*;

fn instantiate_bench(ask: Decimal, bid: Decimal, min_step: Decimal) {
  Orderbook::new(bid, ask, min_step);
}

// rebalance within bounds - ergo; what is the performance hit of a 'rebalance'
// call when a rebalance is not neccassarily required
fn rebalance_in_bounds(ob: Orderbook) {
}

// what is the cost of a rebalance when required?
fn rebalance_out_of_bounds_bench(ob: Orderbook) {
}


fn criterion_benchmark(c: &mut Criterion) {
    let ask: Decimal = Decimal::from_f32(100.0).unwrap();
    let bid: Decimal = Decimal::from_f32(99.0).unwrap();
    let min_step: Decimal = Decimal::from_f32(0.10).unwrap();

    let ob_in_bounds: Orderbook = Orderbook::new(bid, ask, min_step);
    let ob_out_of_bounds: Orderbook = Orderbook::new(bid_b, ask_b, min_step_b);

    c.bench_function("instantiate_bench", |b| b.iter(|| instantiate_bench(ask, bid, min_step)));
    c.bench_function("rebalance_bench", |b| b.iter(|| rebalance_bench(my_ob)));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);