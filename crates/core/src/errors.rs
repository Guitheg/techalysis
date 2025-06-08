use crate::types::Float;

#[derive(Debug)]
pub enum TechalysisError {
    BadParam(String),
    InsufficientData,
    DataNonFinite(String),
    Overflow(usize, Float),
    NotImplementedYet,
}
