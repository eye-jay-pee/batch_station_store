use super::{Anchor, CenterLineElement};
mod error;
pub use error::LineResult;
use std::fmt;
#[derive(Copy, Default, Clone)]
pub struct Line {
    start: Anchor,
    //_azimuth: Angle,
    end: Anchor,
}
impl Line {
    pub fn new(start: Anchor, end: Anchor) -> LineResult<Self> {
        Ok(Self {
            start: start,
            end: end,
        }
        .validate()?)
    }
}
impl CenterLineElement for Line {
    fn get_start(&self) -> Anchor {
        self.start
    }
    fn get_end(&self) -> Anchor {
        self.end
    }
    fn get_length(&self) -> f64 {
        self.end.station - self.start.station
    }
}
impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Line from {}", self.start)?;
        writeln!(f, "...to {}", self.end)?;
        Ok(())
    }
}
