use super::{Angle, CenterLine,Feature, Curve, Point, Station};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
mod record;
pub use record::{Record, RecordError};
mod error;
pub use error::{FileError, FileResult};
impl CenterLine {
    pub fn load(path: &str) -> FileResult<Self> {
        let cl = Self::default();
        let reader = BufReader::new(File::open(path)?);
        let mut buffer: Option<Feature> = None;

        for line in reader.lines() {
            let record = Record::load(&line?);
            match record?.kind {
                Self::LinePoint(s) => {
                    match buffer {
                        None => {
                            let anchor = Anchor::new(record.pos, s);
                            let line = Line::zero_length(anchor)?;
                            buffer = Some(Feature::LineSeg(l));
                        },            

                    }
                },
                Self::PointOfCurve(s)=> (),
                Self::RadiusPoint(a) => (),
                Self::PointOfTangant(s) =>(),
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
    fn 
}
