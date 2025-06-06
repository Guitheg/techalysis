#[derive(Debug)]
pub enum TechalysisError {
    BadParam(String),
    InsufficientData,
    UnexpectedNan,
    NotImplementedYet,
}
