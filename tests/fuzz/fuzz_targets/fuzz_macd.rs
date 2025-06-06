#![no_main]

use libfuzzer_sys::fuzz_target;
use techalysis::indicators::macd::macd;

fuzz_target!(|data: (Vec<f64>, u8, u8, u8)| {
    let (values, fastperiod, slowperiod, signalperiod) = data;
    let fastperiod = (fastperiod as usize % values.len().saturating_add(1)).max(1);
    let slowperiod = (slowperiod as usize % values.len().saturating_add(1)).max(1);
    let signalperiod = (signalperiod as usize % values.len().saturating_add(1)).max(1);
    let _ = macd(&values, fastperiod, slowperiod, signalperiod);
});
