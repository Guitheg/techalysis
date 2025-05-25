const CASES: &[(usize, usize)] = &[(1_000_000, 100), (50_000, 30)];

#[macro_export]
macro_rules! bench_feature {
    (
        $id_name:ident,
        $func:ident
        $(, $arg_name:ident : $arg_ty:ty = $value:expr ),* $(,)?
    ) => {
        paste::paste! {
            fn [<bench_ $func _ $id_name>](c: &mut criterion::Criterion) {
                use rand::{Rng, SeedableRng};
                use rand::rngs::StdRng;

                for &(len, period) in CASES {
                    let mut rng = StdRng::seed_from_u64(period as u64);
                    let data: Vec<f64> = (0..len).map(|_| rng.random_range(0.0..100.0)).collect();
                    let args = (
                        $(
                            concat!(
                                stringify!($arg_name), ": ",
                                stringify!($value)
                            )
                        ),*
                    );
                    let id = format!(
                        "{}(len={}, period={}, args={:?})",
                        stringify!($func), len, period,
                        args
                    );

                    c.bench_function(&id, |b| {
                        let data = &data;
                        b.iter(|| $func(
                            data,
                            period
                            $( , $value )*
                        ))
                    });
                }
            }
        }
    };
}

use technicalysis::indicators::ema;
bench_feature!(
    smoothing_2,
    ema,
    smoothing: f64 = 2.0,
);

use technicalysis::indicators::sma;
bench_feature!(basic, sma);

use technicalysis::indicators::rsi;
bench_feature!(basic, rsi);

criterion::criterion_group!(
    benches,
    bench_ema_smoothing_2,
    bench_sma_basic,
    bench_rsi_basic
);
criterion::criterion_main!(benches);
