use crate::errors::TechalysisError;
use crate::types::Float;

#[derive(Debug)]
pub struct RsiResult {
    pub values: Vec<Float>,
    pub state: RsiState,
}

impl From<RsiResult> for Vec<Float> {
    fn from(result: RsiResult) -> Self {
        result.values
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RsiState {
    pub rsi: Float,
    pub prev_value: Float,
    pub avg_gain: Float,
    pub avg_loss: Float,
    pub period: usize,
}

impl RsiState {
    pub fn next(&self, new_value: Float) -> Result<RsiState, TechalysisError> {
        rsi_next(
            new_value,
            self.prev_value,
            self.avg_gain,
            self.avg_loss,
            self.period,
        )
    }
}

#[inline(always)]
fn calculate_rsi(avg_gain: Float, avg_loss: Float) -> Float {
    if avg_loss == 0.0 {
        if avg_gain == 0.0 {
            return 50.0;
        }
        return 100.0;
    }
    let rs = avg_gain / avg_loss;
    100.0 - (100.0 / (1.0 + rs))
}

pub fn rsi(data_array: &[Float], window_size: usize) -> Result<RsiResult, TechalysisError> {
    let size: usize = data_array.len();
    let mut output = vec![0.0; size];
    let rsi_state = rsi_into(data_array, window_size, output.as_mut_slice())?;
    Ok(RsiResult {
        values: output,
        state: rsi_state,
    })
}

pub fn rsi_into(
    data: &[Float],
    period: usize,
    output_rsi: &mut [Float],
) -> Result<RsiState, TechalysisError> {
    let len = data.len();
    let period_as_float = period as Float;
    if period == 0 || period + 1 > len {
        return Err(TechalysisError::InsufficientData);
    }

    if period == 1 {
        return Err(TechalysisError::BadParam(
            "RSI window size must be greater than 1".to_string(),
        ));
    }

    if output_rsi.len() != len {
        return Err(TechalysisError::BadParam(
            "Output RSI length must match input data length".to_string(),
        ));
    }

    let mut avg_gain: Float = 0.0;
    let mut avg_loss: Float = 0.0;
    output_rsi[0] = Float::NAN;
    for i in 1..=period {
        let delta = data[i] - data[i - 1];
        if !delta.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data[{}] = {:?}",
                i, data[i]
            )));
        }
        if delta > 0.0 {
            avg_gain += delta;
        } else {
            avg_loss -= delta;
        }
        output_rsi[i] = Float::NAN;
    }
    avg_gain /= period_as_float;
    avg_loss /= period_as_float;
    output_rsi[period] = calculate_rsi(avg_gain, avg_loss);
    if !output_rsi[period].is_finite() {
        return Err(TechalysisError::Overflow(period, output_rsi[period]));
    }

    for i in (period + 1)..len {
        let delta = data[i] - data[i - 1];
        if !delta.is_finite() {
            return Err(TechalysisError::DataNonFinite(format!(
                "data[{}] = {:?}",
                i, data[i]
            )));
        }
        (output_rsi[i], avg_gain, avg_loss) =
            rsi_next_unchecked(data[i] - data[i - 1], avg_gain, avg_loss, period_as_float);
        if !output_rsi[i].is_finite() {
            return Err(TechalysisError::Overflow(i, output_rsi[i]));
        }
    }
    Ok(RsiState {
        rsi: output_rsi[len - 1],
        prev_value: data[len - 1],
        avg_gain,
        avg_loss,
        period,
    })
}

pub fn rsi_next(
    new_value: Float,
    prev_value: Float,
    prev_avg_gain: Float,
    prev_avg_loss: Float,
    period: usize,
) -> Result<RsiState, TechalysisError> {
    if period <= 1 {
        return Err(TechalysisError::BadParam(
            "RSI period must be greater than 1".to_string(),
        ));
    }

    if !new_value.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "new_value = {new_value:?}",
        )));
    }
    if !prev_value.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "prev_value = {prev_value:?}",
        )));
    }
    if !prev_avg_gain.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "prev_avg_gain = {prev_avg_gain:?}",
        )));
    }
    if !prev_avg_loss.is_finite() {
        return Err(TechalysisError::DataNonFinite(format!(
            "prev_avg_loss = {prev_avg_loss:?}",
        )));
    }

    let (rsi, avg_gain, avg_loss) = rsi_next_unchecked(
        new_value - prev_value,
        prev_avg_gain,
        prev_avg_loss,
        period as Float,
    );
    if !rsi.is_finite() {
        return Err(TechalysisError::Overflow(0, rsi));
    }
    Ok(RsiState {
        rsi,
        prev_value: new_value,
        avg_gain,
        avg_loss,
        period,
    })
}

#[inline(always)]
pub fn rsi_next_unchecked(
    delta: Float,
    prev_avg_gain: Float,
    prev_avg_loss: Float,
    period: Float,
) -> (Float, Float, Float) {
    let k = 1.0 / period;
    let one_minus_k = 1.0 - k;
    let (avg_gain, avg_loss) = if delta > 0.0 {
        (
            prev_avg_gain * one_minus_k + delta * k,
            prev_avg_loss * one_minus_k,
        )
    } else if delta < 0.0 {
        (
            prev_avg_gain * one_minus_k,
            prev_avg_loss * one_minus_k - delta * k,
        )
    } else {
        (prev_avg_gain * one_minus_k, prev_avg_loss * one_minus_k)
    };

    (calculate_rsi(avg_gain, avg_loss), avg_gain, avg_loss)
}
