use super::file::FileError;
use std::result::Result;
#[derive(Debug)]
pub enum CLError {
    File(FileError),
}
pub type CLResult<T> = Result<T, CLError>;
impl From<FileError> for CLError {
    fn from(err: FileError) -> Self {
        CLError::File(err)
    }
}
