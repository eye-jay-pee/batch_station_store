use std::f64::consts::PI;
#[derive(Debug, Clone, Copy)]
pub struct Radians(f64);
impl Radians {
    pub fn new(value: f64) -> Self {
        Self(value).normalize()
    }
    pub fn as_f64(self) -> f64 {
        self.0
    }
    pub fn normalize(mut self) -> Self {
        self.0 %= 2.0 * PI;
        self
    }
}
mod traits {
    use super::Radians;
    mod format {
        use super::Radians;
        use std::fmt::{Display, Formatter, Result};
        impl Display for Radians {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                write!(f, "{}", f64::from(*self))
            }
        }
    }
    mod convert {
        use super::Radians;
        mod floatf4 {
            use super::Radians;
            impl From<Radians> for f64 {
                fn from(r: Radians) -> Self {
                    r.as_f64()
                }
            }
            impl From<f64> for Radians {
                fn from(f: f64) -> Self {
                    Self::new(f)
                }
            }
        }
        mod float32 {
            use super::Radians;
            impl From<Radians> for f32 {
                fn from(r: Radians) -> Self {
                    r.as_f64() as f32
                }
            }
            impl From<f32> for Radians {
                fn from(f: f32) -> Self {
                    Self::new(f.into())
                }
            }
        }
    }
}
