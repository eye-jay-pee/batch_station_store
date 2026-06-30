use super::RecordError;
use std::result::Result;
#[derive(Debug)]
pub enum FileError {
    IO(std::io::Error),
    UnreadableRecord(RecordError),
    Expected,
}
pub type FileResult<T> = Result<T, FileError>;
impl From<std::io::Error> for FileError {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}
impl From<RecordError> for FileError {
    fn from(re: RecordError) -> Self {
        Self::UnreadableRecord(re)
    }
}
