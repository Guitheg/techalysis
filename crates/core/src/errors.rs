use crate::types::Float;

/// Techalib error types
/// ---
/// This enum defines the various error types that can occur
/// during the execution of the Techalib library.
///
/// Variants
/// ---
/// - `BadParam(String)`: Indicates that a parameter passed to a function is invalid.
/// - `InsufficientData`: Indicates that there is not enough data to perform a calculation.
/// - `DataNonFinite(String)`: Indicates that a data point is not finite (e.g., NaN or Infinity).
/// - `Overflow(usize, Float)`: Indicates that an overflow occurred at a specific index.
/// - `NotImplementedYet`: Indicates that a feature or function is not yet implemented.
#[derive(Debug)]
pub enum TechalibError {
    /// Indicates that a parameter passed to a function is invalid.
    BadParam(String),
    /// Indicates that there is not enough data to perform a calculation.
    InsufficientData,
    /// Indicates that a data point is not finite (e.g., NaN or Infinity).
    DataNonFinite(String),
    /// Indicates that an overflow occurred at a specific index.
    Overflow(usize, Float),
    /// Indicates that a feature or function is not yet implemented.
    NotImplementedYet,
}

impl TechalibError {
    #[inline(always)]
    pub(crate) fn check_same_length(
        data1: (&str, &[Float]),
        data2: (&str, &[Float]),
    ) -> Result<(), TechalibError> {
        if data1.1.len() != data2.1.len() {
            Err(TechalibError::BadParam(format!(
                "Data lengths must match: ({} length = {}) and ({} length = {})",
                data1.0,
                data1.1.len(),
                data2.0,
                data2.1.len()
            )))
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    pub(crate) fn check_finite_at(index: usize, data: &[Float]) -> Result<(), TechalibError> {
        if !data[index].is_finite() {
            Err(TechalibError::DataNonFinite(format!(
                "data[{}] = {:?}",
                index, data[index]
            )))
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    pub(crate) fn check_finite(value: Float) -> Result<(), TechalibError> {
        if !value.is_finite() {
            Err(TechalibError::DataNonFinite(format!("value = {:?}", value)))
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    pub(crate) fn check_overflow_at(index: usize, data: Float) -> Result<(), TechalibError> {
        TechalibError::check_finite_at(index, &[data])
            .map_err(|_| TechalibError::Overflow(index, data))
    }

    #[inline(always)]
    pub(crate) fn check_overflow(value: Float) -> Result<(), TechalibError> {
        TechalibError::check_overflow_at(0, value)
    }
}
