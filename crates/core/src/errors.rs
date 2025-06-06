#[derive(Debug)]
pub enum TechnicalysisError {
    BadParam(String),
    InsufficientData,
    UnexpectedNan,
    NotImplementedYet,
}
