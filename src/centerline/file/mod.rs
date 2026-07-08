use super::{Anchor, Angle, Station};
use std::{
    collections::VecDeque,
    //fs::File,
    //io::{BufRead, BufReader},
    path::PathBuf,
};
mod record;
pub use record::{Record, RecordError};
mod error;
pub use error::{/*CenterLineFileResult,*/ FileError};
/// represents the contents of a particular centerline file. Used for loading and
/// Saving a centerline to carlson's format.
pub struct CenterLineFile {
    records: VecDeque<Record>,
    _address: PathBuf,
}
impl CenterLineFile {
    // /// parse a file, put the contents into a centerline file struct
    // pub fn load(file_path: PathBuf) -> CenterLineFileResult<Self> {
    //     let mut cl_file = Self::new(file_path);
    //     for line in reader.lines() {
    //         cl_file.records.push_back(Record::load(&line?)?);
    //     }
    //     Ok(cl_file)
    // }
    // //pub fn next(&mut self) -> Option
}
impl CenterLineFile {
    pub fn new(path: PathBuf) -> Self {
        Self {
            // maybe should add some path validation here?
            _address: path,
            records: VecDeque::new(),
        }
    }
    pub fn pop_front(&mut self) -> Option<Record> {
        self.records.pop_front()
    }
    pub fn peak_front(&mut self) -> Option<&Record> {
        self.records.front()
    }
}
