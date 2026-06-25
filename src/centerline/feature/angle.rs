use std::fmt;
#[derive(Clone)]
pub struct Angle {
    decimal_degrees: f64,
}
#[allow(dead_code)]
impl Angle {
    const FULL_CIRCLE: f64 = 360.0;
    fn degrees(&self) -> f64 {
        self.decimal_degrees.trunc()
    }
    fn minutes(&self) -> f64 {
        (self.decimal_degrees.fract() * 60.0).trunc()
    }
    fn seconds(&self) -> f64 {
        self.decimal_degrees.fract() * 60.0 * 60.0
    }
}
impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{:3}°{:02}'{:08.5}\"",
            self.degrees(),
            self.minutes(),
            self.seconds()
        )
    }
}
impl From<f64> for Angle {
    fn from(f: f64) -> Self {
        Self {
            decimal_degrees: f % Self::FULL_CIRCLE,
        }
    }
}
