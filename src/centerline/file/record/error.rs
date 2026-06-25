use super::FieldError;
use std::result::Result;
#[derive(Debug)]
pub enum RecordError {
    TooManyFields(usize),
    TooFewFields(usize),
    InvalidField(FieldError),
    UnknownKind(String),
}
pub type RecordResult<T> = Result<T, RecordError>;
impl From<FieldError> for RecordError {
    fn from(field_e: FieldError) -> Self {
        Self::InvalidField(field_e)
    }
}
