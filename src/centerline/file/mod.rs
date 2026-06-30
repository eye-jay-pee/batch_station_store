use super::{Angle, CenterLine, Curve, Feature, Point, Station};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{PathBuf},
};
mod record;
pub use record::{Record, RecordError};
mod error;
pub use error::{FileError, FileResult};
/// represents the contents of a particular centerline file. Used for loading and 
/// Saving a centerline to carlson's format.
pub struct CenterLineFile {
    records: VecDeque<Record>,
            // its used to save the file, so it IS needed. removing it for now
            // seems like a good idea, until you forget why this cant impl default
            // and you cahnge it back only to rememeber you're going in circles.
            // trust me 
    _address: PathBuf,
}
impl CenterLineFile {
    /// parse a file, put the contents into a centerline file struct
    pub fn load(file_path: PathBuf) -> CenterLineFileResult<Self> {
        let mut cl_file = Self::new(file_path)  ;
        for line in reader.lines() {
            cl_file.records.push_back(Record::load(&line?)?);
        }
        Ok(cl_file)
    }
}
impl CenterLineFile {
    pub fn new(path: PathBuf) {
        Self {
            // maybe should add some path validation here? 
            ._address: path,
            .records: VecDeque::new(),

        }
    }
    pub fn deque_record(&mut self) -> Record {
        match self.records.pop_front() {
            Some(r) => r,
            None => Record::End,
        }

        // rertunr the terminoator record if end of list is reached. 
        // how to cehck if end of list has been reached?
        // be sure the parser actually detects null terminator reocords (it dont rn)
    }
}
