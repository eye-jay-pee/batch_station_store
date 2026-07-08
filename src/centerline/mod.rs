mod error;
pub use error::CenterLineResult;
mod file;
pub use file::{CenterLineFile, FileError};
mod primitives;
use primitives::{Anchor, Angle, /*Curve,*/ Point, Station};
mod element;
use element::CenterLineElement;

use std::collections::VecDeque;
use std::path::PathBuf;

#[derive(Clone)]
pub struct CenterLine {
    elements: Vec<Box<dyn CenterLineElement>>,
    file: CenterLineFile,
}
impl CenterLineElement for CenterLine {
    fn get_start(&self) -> Anchor {
        self.elements.peak_front().get_start()
    }
    fn get_end(&self) -> Anchor {
        self.elements.peak_back().get_end()
    }
    fn get_length(&self) -> f64 {
        self.get_end().station - self.get_start().station
    }
}
impl CenterLine {
    pub fn new(path: &str) -> CenterLineResult<Self> {
        Ok(Self {
            elements: VecDeque::new(),
            file: CenterLineFile::load(PathBuf::from(path))?,
        })
    }
    pub fn push_back<T, CenterLineElement>(
        &mut self,
        element: T,
    ) -> CenterLineResult<()> {
        self.elements.push_back(element);
        self.validate()
    }
    pub fn push_front<T, CenterLineElement>(
        &mut self,
        element: T,
    ) -> CenterLineResult<()> {
        self.elements.push_front(element);
        self.validate()
    }
}
