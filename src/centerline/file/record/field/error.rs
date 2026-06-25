use std::result::Result;
#[derive(Debug)]
pub enum FieldError {
    ExpectedUint,
    ExpectedInt,
    ExpectedNumber,
}
pub type FieldResult<T> = Result<T, FieldError>;
