mod station;
pub use station::Station;
mod point;
pub use point::Point;
mod anchor;
pub use anchor::Anchor;
mod curve;
pub use curve::Curve;
mod angle;
pub use angle::Angle;

#[derive(PartialOrd)]
pub enum Element {
    LineSeg(Line),
    CurveSeg(Curve),
}

impl Feature {
    pub fn load_line(a: Anchor, b:Anchor) -> FeatureResult<Self> {
      
    }
    pub fn load_curve(a: Anchor, b: Anchor, around: Point, delta: Angle) -> 

}
impl Feature {
    pub fn get_start(&self) -> Anchor {
        match self {
            Self::LineSeg(l) => l.start,
            Self::CurveSeg(c) => c.start,
        }
    }
    pub fn get_end(&self) -> Anchor {
        match self {
            Self::LineSeg(l) => l.end,
            Self::CurveSeg(c) => c.end,
        }
    }
    pub fn get_length(&self) -> f64 {
        match self {
            Self::LineSeg(l) => l.get_length(),
            Self::CurveSeg(c) => c.get_length(),
        }
    }
}

