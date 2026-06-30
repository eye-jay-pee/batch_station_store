use super::{Point, Station};
use std::fmt;

#[derive(Copy, Clone, Default)]
pub struct Anchor {
    sta: Station,
    pt: Point,
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
