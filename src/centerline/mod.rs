mod error;
pub use error::CLResult;

mod file;
//pub use file::FileResult;

mod feature;
use feature::{Anchor, Angle, Curve, Point, Station};

#[derive(Clone, Default)]
pub struct CenterLine {
    start: Anchor,
    curves: Vec<Curve>,
    end: Anchor,
}
