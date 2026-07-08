mod sexagesimal;
pub use sexagesimal::Sexagesimal;
#[derive(Debug, Clone, Copy)]
pub enum Degrees {
    IEEE64(f64),
    IEEE32(f32),
    Babylon60(Sexagesimal),
}
impl Degrees {
    pub fn as_float64(self) -> f64 {
        match self {
            Self::IEEE64(d) => d,
            Self::IEEE32(d) => d as f64,
            Self::Babylon60(d) => d.into(),
        }
    }
    pub fn as_float32(self) -> f32 {
        match self {
            Self::IEEE64(d) => d as f64,
            Self::IEEE32(d) => d,
            Self::Babylon60(d) => d.into(),
        }
    }
    pub fn as_sexagesimal(self) -> Sexagesimal {
        match self {
            Self::IEEE64(d) => d.into(),
            Self::IEEE32(d) => d.into(),
            Self::Babylon60(d) => d,
        }
    }
    pub fn new_float64(degrees: f64) -> Self {
        Self::IEEE64(degrees)
    }
    pub fn new_float32(degrees: f32) -> Self {
        Self::IEEE32(degrees)
    }
    pub fn new_sexagesimal(degrees: Sexagesimal) -> Self {
        Self::Babylon60(degrees)
    }
}
mod traits {
    use super::{Degrees, Sexagesimal};
    mod fmt {
        use super::Degrees;
        use std::fmt::{Display, Formatter, Result};
        impl Display for Degrees {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                let theta = '\u{03B8}';
                match self {
                    Self::Dec(dd) => write!(f, "{}={}", theta, dd),
                    Self::Sex(sd) => write!(f, "{}={}", theta, sd),
                }
            }
        }
    }
    mod convert {
        use super::{Degrees, Sexagesimal};
        mod float64 {
            use super::Degrees;
            impl From<f64> for Degrees {
                fn from(degrees: f64) -> Self {
                    Self::new_float64(degrees)
                }
            }
            impl From<Degrees> for f64 {
                fn from(degrees: Degrees) -> Self {
                    degrees.as_float64()
                }
            }
        }
        mod float32 {
            use super::Degrees;
            impl From<f32> for Degrees {
                fn from(degrees: f32) -> Self {
                    Self::new_f32(degrees)
                }
            }
            impl From<Degrees> for f32 {
                fn from(degrees: Degrees) -> Self {
                    degrees.as_float32()
                }
            }
        }
        mod sexagesimal {
            use super::{Degrees, Sexagesimal};
            impl From<Sexagesimal> for Degrees {
                fn from(deg_min_sec: Sexagesimal) -> Self {
                    Self::new_sexagesimal(deg_min_sec)
                }
            }
            impl From<Degrees> for Sexagesimal {
                fn from(degrees: Degrees) -> Self {
                    degrees.as_sexagesimal()
                }
            }
        }
    }
    mod ops {
        use super::Degrees;
        mod mul {
            use super::Degrees;
            use std::ops::Mul;
            impl Mul<f64> for Degrees {
                type Output = Self;
                fn mul(self, rhs: f64) -> Self {
                    Self::from(self.into() * rhs)
                }
            }
            impl Mul<f32> for Degrees {
                type Output = Self;
                fn mul(self, rhs: f32) -> Self {
                    Self::from(self.into() * rhs)
                }
            }
        }
        mod div {
            use super::Degrees;
            use std::ops::Div;
            impl Div<f64> for Degrees {
                type Output = Self;
                fn div(self, rhs: f64) -> Self {
                    Self::from(self.into() / rhs)
                }
            }
            impl Div<f32> for Degrees {
                type Output = Self;
                fn div(self, rhs: f32) -> Self {
                    Self::from(self.into() / rhs)
                }
            }
        }
    }
}
