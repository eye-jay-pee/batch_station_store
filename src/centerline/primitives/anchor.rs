use super::{Point, Station};
use std::fmt;

#[derive(Debug, Copy, Clone, Default)]
pub struct Anchor {
    pub station: Station,
    pub pt: Point,
}
impl Anchor {
    pub fn new(point: Point, station: Station) -> Self {
        Self {
            sta: station,
            pt: point,
        }
    }
}
impl fmt::Display for Anchor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} @", self.sta)?;
        writeln!(f, "{}", self.pt)
    }
}
