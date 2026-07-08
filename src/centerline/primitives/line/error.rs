use super::{CenterLineElement, Line};
use std::result::Result;
#[derive(Debug)]
pub enum LineError {
    NegativeLength,
    LengthConflict,
}
pub type LineResult<T> = Result<T, LineError>;
impl Line {
    pub fn validate(self) -> LineResult<Self> {
        if self.get_length() < 0.0 {
            Err(LineError::NegativeLength)
        } else {
            let station_dist = self.end.station - self.start.station;
            let point_dist = self.end.pt - self.start.pt;
            if station_dist != point_dist {
                Err(LineError::LengthConflict)
            } else {
                Ok(self)
            }
        }
    }
}
