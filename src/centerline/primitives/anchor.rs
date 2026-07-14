use super::{Point, Station};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Default, Deserialize, Serialize)]
pub struct Anchor {
    pub station: Station,
    pub point: Point,
}
#[allow(dead_code)]
impl Anchor {
    pub fn new(point: Point, station: Station) -> Self {
        Self { station, point }
    }
}
mod traits {
    use super::Anchor;
    mod format {
        use super::Anchor;
        use std::fmt::{Display, Formatter, Result};
        impl Display for Anchor {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                writeln!(f, "{} @", self.station)?;
                writeln!(f, "{}", self.point)
            }
        }
    }
}
