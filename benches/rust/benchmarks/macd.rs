use criterion::BenchmarkId;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use technicalysis::indicators::macd;

fn bench_macd(c: &mut criterion::Criterion) {
    let mut bench_group = c.benchmark_group("macd");

    let cases = vec![(50_000, 12, 26, 9), (1_000_000, 26, 52, 18)];

    for (len, fastperiod, slowperiod, signalperiod) in cases {
        let mut rng = StdRng::seed_from_u64(fastperiod as u64);
        let data: Vec<f64> = (0..len).map(|_| rng.random_range(0.0..100.0)).collect();

        bench_group.bench_with_input(
            BenchmarkId::new(
                format!("len={len}"),
                format!("{fastperiod}-{slowperiod}-{signalperiod}"),
            ),
            &(fastperiod, slowperiod, signalperiod),
            |b, &(fastperiod, slowperiod, signalperiod)| {
                b.iter(|| {
                    let _ = macd(&data, fastperiod, slowperiod, signalperiod);
                })
            },
        );
    }
}

criterion::criterion_group!(bench, bench_macd);
