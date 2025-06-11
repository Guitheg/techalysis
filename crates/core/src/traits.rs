use crate::errors::TechalysisError;

/// State trait
/// ---
/// This trait defines the interface for a state that can be updated with new samples.
/// It is used to incrementally update the state with new data points.
pub trait State<T> {
    /// Update the state with a new sample
    fn update(&mut self, sample: T) -> Result<(), TechalysisError>;
}
