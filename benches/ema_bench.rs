use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use technicalysis::features::ema::ema;

fn ema_bench(c: &mut Criterion) {
    let base_id = "ema";
    let mut rng = rand::thread_rng();

    let data: Vec<f64> = (0..1_000_000).map(|_| rng.gen_range(0.0..100.0)).collect();
    let period = 100;
    let id = format!(
        "{} (length: {}, window size: {}",
        base_id,
        data.len(),
        period
    );
    c.bench_function(&id, |b| b.iter(|| ema(&data, period, 2f64)));

    let data: Vec<f64> = (0..50_000).map(|_| rng.gen_range(0.0..100.0)).collect();
    let period = 30;
    let id = format!(
        "{} (length: {}, window size: {}",
        base_id,
        data.len(),
        period
    );
    c.bench_function(&id, |b| b.iter(|| ema(&data, period, 2f64)));
}

criterion_group!(benches, ema_bench);
criterion_main!(benches);
