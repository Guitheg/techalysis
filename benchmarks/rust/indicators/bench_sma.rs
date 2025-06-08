use criterion::BenchmarkId;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use techalysis::indicators::sma::sma;
use techalysis::types::Float;

fn bench_sma(c: &mut criterion::Criterion) {
    let mut bench_group = c.benchmark_group("sma");

    let cases = vec![(50_000, 12), (1_000_000, 30)];

    for (len, period) in cases {
        let mut rng = StdRng::seed_from_u64(period as u64);
        let data: Vec<Float> = (0..len).map(|_| rng.random_range(0.0..100.0)).collect();

        bench_group.bench_with_input(
            BenchmarkId::new(format!("len={len}"), period),
            &period,
            |b, &period| {
                b.iter(|| {
                    let _ = sma(&data, period);
                })
            },
        );
    }
}

criterion::criterion_group!(bench, bench_sma);
