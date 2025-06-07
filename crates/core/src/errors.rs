#[derive(Debug)]
pub enum TechalysisError {
    BadParam(String),
    InsufficientData,
    DataNonFinite(String),
    NotImplementedYet,
}
