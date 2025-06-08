pub(crate) mod indicators;

criterion::criterion_main! {
indicators::bench_tema::bench,indicators::bench_dema::bench,indicators::bench_wma::bench,    indicators::bench_bbands::bench,
    indicators::bench_ema::bench,
    indicators::bench_sma::bench,
    indicators::bench_rsi::bench,
    indicators::bench_macd::bench
}
