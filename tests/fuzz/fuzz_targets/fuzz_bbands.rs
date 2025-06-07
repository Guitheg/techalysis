#![no_main]

use libfuzzer_sys::fuzz_target;
use techalysis::indicators::bbands::{bbands, BBandsMA};

fuzz_target!(|data: (Vec<f64>, u8, f64, f64)| {
    let (data, period, std_up, std_down) = data;
    let period = (period as usize % data.len().saturating_add(1)).max(1);
    let _ = bbands(&data, period, std_up.into(), std_down.into(), BBandsMA::SMA);
});
