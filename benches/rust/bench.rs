pub(crate) mod benchmarks;

criterion::criterion_main! {
    benchmarks::ema::bench,
    benchmarks::sma::bench,
    benchmarks::rsi::bench,
    benchmarks::macd::bench
}
