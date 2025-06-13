#![no_main]

use libfuzzer_sys::fuzz_target;
use techalib::{indicators::midpoint::midpoint, types::Float};

fuzz_target!(|data: (Vec<Float>, u8)| {
    let (v, w) = data;
    let w = (w as usize % v.len().saturating_add(1)).max(1);
    let _ = midpoint(&v, w);
});
