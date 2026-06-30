use super::Anchor;
use std::fmt;
#[derive(Copy, Clone, Default)]
pub struct Line {
    start: Anchor,
    azimuth: Angle,

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

#[allow(dead_code)]
impl Line {
    pub fn set_start(&mut self, start: Anchor) -> LineResult<()> {
        self.start = start;
        self.validate()
    }
    pub fn zero_length(start_and_end: Anchor) -> LineResult<Self> {}

    pub fn new(start: Anchor, end: Anchor) -> LineResult<Self> {
        // check that the length matches station diff
    }
    fn endpoint_distance(&self) -> f64 {
        end.pt - start.pt
    }
    fn length(&self) -> f64 {
        end.sta - start.sta
    }
}
impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Line from {}", self.start)?;
        writeln!(f, "...to {}", self.end)?;
        Ok(())
    }
}
