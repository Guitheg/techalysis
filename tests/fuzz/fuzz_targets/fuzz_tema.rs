#![no_main]

use libfuzzer_sys::fuzz_target;
use techalysis::{indicators::tema::tema, types::Float};

fuzz_target!(|data: (Vec<Float>, u8, Float)| {
    let (v, w, s) = data;
    let w = (w as usize % v.len().saturating_add(1)).max(1);
    let _ = tema(&v, w, s.into());
});
