use super::errors::TechnicalysisError;

const MINIMAL_PERIOD_VALUE: usize = 2;
const MAXIMAL_PERIOD_VALUE: usize = 100_000;

pub fn lookback(period: usize) -> Result<usize, TechnicalysisError> {
    if period < MINIMAL_PERIOD_VALUE || period > MAXIMAL_PERIOD_VALUE {
        Err(TechnicalysisError::BadParam(format!(
            "period must be between {MINIMAL_PERIOD_VALUE} and {MAXIMAL_PERIOD_VALUE}"
        )))
    } else {
        Ok(period - 1)
    }
}
