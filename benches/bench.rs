const CASES: &[(usize, usize)] = &[(1_000_000, 100), (50_000, 30)];

#[macro_export]
macro_rules! bench_feature {
    (
        $id_name:ident,
        $func:ident,
        $( $arg_name:ident : $arg_ty:ty = $value:expr ),* $(,)?
    ) => {
        paste::paste! {
            fn [<bench_ $func _ $id_name>](c: &mut criterion::Criterion) {
                use rand::{Rng, SeedableRng};
                use rand::rngs::StdRng;

                for &(len, period) in CASES { // Need to be defined on the caller side
                    let mut rng = StdRng::seed_from_u64(period as u64);
                    let data: Vec<f64> = (0..len).map(|_| rng.gen_range(0.0..100.0)).collect();
                    let id = format!(
                        "{}(len={}, period={}, args={:?})",
                        stringify!($func), len, period,
                        [ $( concat!(stringify!($arg_name), ": ", $value) ),* ]
                    );

                    c.bench_function(&id, |b| {
                        let data = &data;
                        #[allow(non_snake_case)]
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

use technicalysis::features::ema::ema;
bench_feature!(
    without_nan,
    ema,
    smoothing: f64 = 2.0,
    handle_nan: bool = false,
);

use technicalysis::features::sma::sma;
bench_feature!(
    without_nan,
    sma,
    handle_nan: bool = false,
);

bench_feature!(
    with_nan,
    sma,
    handle_nan: bool = true,
);

criterion::criterion_group!(
    benches,
    bench_ema_without_nan,
    bench_sma_without_nan,
    bench_sma_with_nan
);
criterion::criterion_main!(benches);
