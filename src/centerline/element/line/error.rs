use super::Line;
use std::result::Result;
#[derive(Debug)]
pub enum LineError {
    NegativeLength,
    LengthConflict,
}
type LineResult<T> = Result<T, LineError>;
impl Line {
    pub fn validate(&self) -> LineResult<()> {
        if self.length < 0 {
            Err(LineError::NegativeLength)
        } else {
            let station_dist = end.sta - start.sta;
            let point_dist = end.pt - start.pt;
            if station_dist != point_dist {
                Err(LineError::LengthConflict)
            } else {
                Ok()
            }
        }
    }
}
