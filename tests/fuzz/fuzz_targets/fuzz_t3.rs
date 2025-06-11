#![no_main]

use libfuzzer_sys::fuzz_target;
use techalysis::{indicators::t3::t3, types::Float};

fuzz_target!(|data: (Vec<Float>, u8, Float, Option<Float>)| {
    let (input, period, vfactor, alpha) = data;
    let period = (period as usize % input.len().saturating_add(1)).max(1);
    let _ = t3(&input, period, vfactor, alpha.into());
});
