mod error;
pub use error::CLResult;

mod file;
//pub use file::FileResult;

mod element;
use element::{Anchor, Angle, Curve, Point, Station};

#[derive(Clone)]
pub struct CenterLine {
    elements: VecDeque<CenterLineElement>,
    file: CenterLineFile,
}
impl CenterLine {
    pub fn load(path: &str) -> FileResult<Self> {
        let cl = Self::new(path);
        loop {
            let next_record = cl_file.deque();
            match cl_file.front() {
                None => break,
                Some(Record::End) => break,
                Some(Record::LinePoint(s)) => {
                    let line = Feature::LineSeg(Line::new());
                    

                    
                }
                Some(Record::PointOfCurve(s)) => (),
                Some(Record::RadiusPoint(a)) => (),
                Some(Record::PointOfTangant(s)) => (),
            }
        }
        Ok(cl)
    }
    pub fn save(&self, _path: &str) -> FileResult<()> {
        //let _writer = BufWriter::new(File::open(path)?);

        Ok(())
    }
}
impl CenterLine {
    pub fn new(path: &str) -> CenterLineResult<Self> {
        let cl = Self {
            elements: VecDeque::new(),
            file: CenterLineFile::load(PathBuf::from(path))?,
        }
        Ok(cl)
    }
    pub fn push_back(&mut self, feature: Feature) -> CenterLineResult<()> {
        self.elements.push_back(feature);
        self.validate()
    }
    pub fn push_front(&mut self, feature: Feature) -> CenterlineResult<()> {
        self.elements.push_front(feature);
        self.validate()
    }
}
