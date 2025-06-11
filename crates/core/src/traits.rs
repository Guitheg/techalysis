use crate::errors::TechalysisError;

pub trait State<T> {
    fn update(&mut self, sample: T) -> Result<(), TechalysisError>;
}