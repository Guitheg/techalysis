#![no_main]

use libfuzzer_sys::fuzz_target;
use technicalysis::indicators::ema;

fuzz_target!(|data: (Vec<f64>, u8, f64)| {
    let (v, w, s) = data;
    let w = (w as usize % v.len().saturating_add(1)).max(1);
    let _ = ema(&v, w, s);
});
