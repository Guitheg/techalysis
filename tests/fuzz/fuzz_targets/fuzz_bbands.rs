#![no_main]

use libfuzzer_sys::fuzz_target;
use techalib::indicators::bbands::{bbands, BBandsMA, DeviationMulipliers};
use techalib::types::Float;

fuzz_target!(|data: (Vec<Float>, u8, Float, Float)| {
    let (data, period, std_up, std_down) = data;
    let period = (period as usize % data.len().saturating_add(1)).max(1);
    let _ = bbands(
        &data,
        period,
        DeviationMulipliers {
            up: std_up.into(),
            down: std_down.into(),
        },
        BBandsMA::SMA,
    );
});
