use std::{f64::consts::PI, fmt};
#[derive(Clone)]
pub struct Radians(f64);
impl From<Degrees> for Radians {
    fn from(d: Degrees) -> Self {
        Self(d.0 * PI / 180)
    }
}
impl From<Radians> for f64 {
    fn from(d: Radians) -> Self {
        d.0
    }
}
impl From<f64> for Radians {
    fn from(f: f64) -> Self {
        Self(f)
    }
}
impl fmt::Display for Radians {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", f64::from(self))
    }
}
