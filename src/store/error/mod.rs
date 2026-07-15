/// children
mod traits;

/// imports
use std::io;

#[derive(Debug)]
pub enum StoreError {
    IO(io::Error),
    CSV(csv::Error),
    EOF,
}
pub type StoreResult<T> = Result<T, StoreError>;
