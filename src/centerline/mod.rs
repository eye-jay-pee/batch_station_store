mod error;
pub use error::CenterLineResult;
mod primitives;
use primitives::{Anchor, /*Degrees, Curve,*/ Point, Station};
mod traits;
use traits::CenterLineElement;

use std::collections::VecDeque;

pub struct CenterLine {
    elements: VecDeque<Box<dyn CenterLineElement>>,
}
impl CenterLine {
    pub fn load(path: &str) -> Self {
        Self::new(path).unwrap()
    }
}
impl CenterLineElement for CenterLine {
    fn get_start(&self) -> Option<Anchor> {
        self.elements.front()?.get_start()
    }
    fn get_end(&self) -> Option<Anchor> {
        self.elements.back()?.get_end()
    }
    fn get_length(&self) -> f64 {
        let start = self.get_start();
        let end = self.get_end();
        match (start, end) {
            (Some(s), Some(e)) => e.station - s.station,
            _ => 0.0,
        }
    }
}
impl CenterLine {
    pub fn new(_path: &str) -> CenterLineResult<Self> {
        Ok(Self {
            elements: VecDeque::new(),
            //file: CenterLineFile::load(PathBuf::from(path))?,
        })
    }
    pub fn _push_back<T>(&mut self, element: T) -> CenterLineResult<()>
    where
        T: CenterLineElement + 'static,
    {
        self.elements.push_back(Box::new(element));
        self._validate()
    }
    pub fn _push_front<T>(&mut self, element: T) -> CenterLineResult<()>
    where
        T: CenterLineElement + 'static,
    {
        self.elements.push_front(Box::new(element));
        self._validate()
    }
}
