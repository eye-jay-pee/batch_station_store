use super::Radians;
use std::{f64::consts::PI, fmt};
#[derive(Clone)]
pub struct Degrees(f64);
impl From<Radians> for Degrees {
    fn from(r: Radians) -> Self {
        Self(r.0 * 180 / PI)
    }
}
impl From<Degrees> for f64 {
    fn from(d: Degrees) -> Self {
        d.0
    }
}
impl From<f64> for Degrees {
    fn from(f: f64) -> Self {
        Self(f)
    }
}
impl fmt::Display for Degrees {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\u{00B0}", f64::from(self))
    }
}
