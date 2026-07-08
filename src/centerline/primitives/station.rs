use std::{fmt, ops::Sub};
#[derive(Debug, Copy, Clone, Default)]
pub struct Station {
    value: f64,
}
impl Station {
    fn major(&self) -> f64 {
        (self.value / 100.0).trunc()
    }
    fn minor(&self) -> f64 {
        self.value % 100.0
    }
}
impl fmt::Display for Station {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}+{}", self.major(), self.minor())
    }
}
impl From<f64> for Station {
    fn from(f: f64) -> Self {
        Self { value: f }
    }
}
impl Sub for Station {
    type Output = f64;
    fn sub(self, rhs: Station) -> Self::Output {
        self.value - rhs.value
    }
}
