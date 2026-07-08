use std::fmt;
use std::ops::{Div, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Sexagesimal {
    degrees: i32,
    minutes: i32,
    seconds: f32,
}
impl Sexagesimal {
    pub fn new(degrees: i32, minutes: i32, seconds: f32) -> Self {
        Self {
            degrees: degrees,
            minutes: minutes,
            seconds: seconds,
        }
        .normalize()
    }
    fn normalize_seconds(&mut self) {
        self.minutes += (self.seconds / 60.0).floor() as i32;
        self.seconds = self.seconds.rem_euclid(60.0);
    }
    fn normalize_minutes(&mut self) {
        self.degrees += self.minutes.div_euclid(60);
        self.minutes = self.minutes.rem_euclid(60);
    }
    fn normalize_degrees(&mut self) {
        self.degrees = self.degrees.rem_euclid(60 * 6);
    }
    pub fn normalize(mut self) -> Self {
        self.normalize_seconds();
        self.normalize_minutes();
        self.normalize_degrees();
        self
    }
}
impl Mul<f32> for Sexagesimal {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self::from(f32::from(self) * rhs)
    }
}
impl Mul<f64> for Sexagesimal {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self::from(f64::from(self) * rhs)
    }
}
impl Div<f32> for Sexagesimal {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self::from(f32::from(self) / rhs)
    }
}
impl Div<f64> for Sexagesimal {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self::from(f64::from(self) / rhs)
    }
}
impl From<f32> for Sexagesimal {
    fn from(value: f32) -> Self {
        let fract = value.fract();
        Self::new(
            value.trunc() as i32,
            (fract * 60.0).trunc() as i32,
            (fract * 60.0 * 60.0) as f32,
        )
    }
}
impl From<f64> for Sexagesimal {
    fn from(value: f64) -> Self {
        let fract = value.fract();
        Self::new(
            value.trunc() as i32,
            (fract * 60.0).trunc() as i32,
            (fract * 60.0 * 60.0) as f32,
        )
    }
}
impl From<Sexagesimal> for f32 {
    fn from(babylon: Sexagesimal) -> Self {
        let mut rasta = babylon.degrees as f32;
        rasta += babylon.minutes as f32 / 60.0;
        rasta += babylon.seconds as f32 / 60.0 / 60.0;
        rasta
    }
}
impl From<Sexagesimal> for f64 {
    fn from(babylon: Sexagesimal) -> Self {
        let mut rasta = f64::from(babylon.degrees);
        rasta += babylon.minutes as f64 / 60.0;
        rasta += babylon.minutes as f64 / 60.0 / 60.0;
        rasta
    }
}
impl fmt::Display for Sexagesimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let deg = '\u{00B0}';
        let min = '\u{2032}';
        let sec = '\u{2033}';
        write!(f, "{}{}", self.degrees, deg)?;
        write!(f, "{}{}", self.minutes, min)?;
        write!(f, "{}{}", self.seconds, sec)?;
        Ok(())
    }
}
