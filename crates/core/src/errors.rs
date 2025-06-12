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
