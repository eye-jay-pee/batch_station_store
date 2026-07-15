/// children
mod error;
pub use error::{StoreError, StoreResult};

/// imports
use csv::{Reader, Writer};
use serde::de::DeserializeOwned;
use std::io::{Read, Write};

/// definition
pub trait Store: DeserializeOwned {
    fn load<R: Read>(rdr: Reader<R>) -> StoreResult<Box<Self>>;
    fn save<W: Write>(&self, wtr: Writer<W>) -> StoreResult<()>;
}
