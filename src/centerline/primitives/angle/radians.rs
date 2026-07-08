use std::fmt;
#[derive(Debug, Clone, Copy)]
pub struct Radians(f64);
impl fmt::Display for Radians {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", f64::from(*self))
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
