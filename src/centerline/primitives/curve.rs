use super::{Anchor, CenterLineElement, Point};
use std::fmt;
#[derive(Copy, Clone, Default)]
pub struct Curve {
    start: Anchor,
    center: Point,
    delta_angle: f64,
    end: Anchor,
}
impl CenterLineElement for Curve {
    fn get_start(&self) -> Anchor {
        self.start
    }
    fn get_end(&self) -> Anchor {
        self.end
    }
    fn get_length(&self) -> f64 {
        self.end.station - self.start.station
    }
}
#[allow(dead_code)]
impl Curve {
    fn get_radius(&self) -> f64 {
        self.start.pt - self.center
    }
    fn get_chord(&self) -> f64 {
        self.start.pt - self.end.pt
    }
    fn get_degrees_of_arc(&self) -> f64 {
        5729.578 / self.get_radius()
    }
}
impl fmt::Display for Curve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Curve around {}", self.center)?;
        writeln!(f, "start: {}", self.start)?;
        writeln!(f, "delta: {}", self.start)?;
        writeln!(f, "end: {}", self.end)?;
        Ok(())
    }
}
