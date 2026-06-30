mod degrees;
pub use degrees::Degress;
mod radians;
pub use radians::Radians;

/// have tried to use a pair of options with each possibility
/// ```
/// pub struct Angle {
///     degrees: Option<Degrees>,
///     radians: Option<Radians>,
/// }
/// ```
/// this would allow for cacheing derived values, but doesnt keep record of
/// which record version is the origional one, and which is derived. this is
/// not ideal. If cacheing becomes neccisary, the way to do it will
/// ```
/// pub enum AngleCache {
///     AsDegrees(Degrees),
///     AsRadians(Radians),
/// }
/// pub struct Angle {
///     source: AngleCache,
///     cache: Option<AngleCache>,
/// }
/// ```
/// I think this is overkill for now, but if cacheing becomes a need, this is
/// how it aught to be done.

pub enum Angle {
    As_Degrees(Degrees),
    As_Radians(Radians),
}
mod convert {
    /// Rad -> Angle
    impl From<Radians> for Angle {
        fn from(r: Radians) -> Self {
            Self::as_radians(r)
        }
    }
    /// Angle -> Rad
    impl From<Angle> for Radians {
        fn from(a: Angle) -> Self {
            match self {
                Angle::radians(r) => r,
                Angle::degrees(d) => Radians::from(d),
            }
        }
    }
    /// Deg -> Angle
    impl From<Degrees> for Angle {
        fn from(d: Degrees) -> Self {
            Self {
                degrees: Some(d),
                radians: None,
            }
        }
    }
    /// Angle -> Deg
    impl From<Angle> for Degrees {
        fn from(a: Angle) -> Self {
            Self::angle(a)
        }
    }
}
impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let theta = '\u{03B8}';
        write!(f, "{}={}", theta, Degrees::from(self))
    }
}

impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Radians
        }
        writeln!(
            f,
            "{:3}°{:02}'{:08.5}\"",
            self.degrees(),
            self.minutes(),
            self.seconds()
        )
    }
}
