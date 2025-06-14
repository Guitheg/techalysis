#![no_main]

use libfuzzer_sys::fuzz_target;
use techalib::{indicators::roc::roc, types::Float};

fuzz_target!(|data: (Vec<Float>, u8)| {
    let (v, period) = data;
    let period = (period as usize % v.len().saturating_add(1)).max(1);
    let _ = roc(&v, period);
});
