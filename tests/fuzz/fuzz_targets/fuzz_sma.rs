#![no_main]

use libfuzzer_sys::fuzz_target;
use technicalysis::indicators::sma::sma;

fuzz_target!(|data: (Vec<f64>, u8)| {
    let (v, w) = data;
    let w = (w as usize % v.len().saturating_add(1)).max(1);
    let _ = sma(&v, w);
});
