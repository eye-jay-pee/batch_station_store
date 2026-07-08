mod degrees;
pub use degrees::Degrees;
mod radians;
pub use radians::Radians;

use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub enum Angle {
    Degrees(Degrees),
    Radians(Radians),
}
impl Angle {
    pub fn as_degrees(self) -> Degrees {
        match self {
            Self::Degrees(d) => d,
            Self::Radians(r) => Self::new_degrees(r * 180.0 / PI),
        }
    }
    pub fn as_radians(self) -> Degrees {
        match self {
            Self::Degrees(d) => Self::new_radians(d * PI / 180.0),
            Self::Radians(r) => r,
        }
    }
    pub fn new_degrees(degrees: Degrees) -> Self {
        Self::degrees(degrees)
    }
    pub fn new_radians(radians: Radians) -> Self {
        Self::radians(radians)
    }
}
mod traits {
    use super::{Angle, Degrees, Radians};
    mod format {
        use super::Angle;
        use std::fmt::{Display, Formatter, Result};
        impl Display for Angle {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                let theta = '\u{03B8}';
                match self {
                    Self::Deg(d) => write!(f, "{}={}", theta, d),
                    Self::Rad(r) => write!(f, "{}={}", theta, r),
                }
            }
        }
    }
    mod convert {
        use super::{Angle, Degrees, Radians};
        mod radians {
            use super::{Angle, Radians};
            impl From<Radians> for Angle {
                fn from(radians: Radians) -> Self {
                    Self::new_radians(radians)
                }
            }
            impl From<Angle> for Radians {
                fn from(angle: Angle) -> Self {
                    angle.as_radians()
                }
            }
        }
        mod degrees {
            use super::{Angle, Degrees};
            impl From<Degrees> for Angle {
                fn from(degrees: Degrees) -> Self {
                    Self::new_degrees(degrees)
                }
            }
            impl From<Angle> for Degrees {
                fn from(angle: Angle) -> Self {
                    angle.as_degrees()
                }
            }
        }
    }
}
