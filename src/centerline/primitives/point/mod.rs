//use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};

mod test;
mod traits;

mod error;
#[allow(unused_imports)]
use error::{PointError, PointResult};

#[derive(PartialEq, Debug, Copy, Clone, Default, Deserialize, Serialize)]
pub struct Point {
    pub northing: f64,
    pub easting: f64,
}
impl Point {
    pub fn new(northing: f64, easting: f64) -> Self {
        Self { northing, easting }
    }
}
impl Point {
    pub fn distance_to(&self, dest: &Self) -> f64 {
        let ndif = self.northing - dest.northing;
        let edif = self.easting - dest.easting;
        (ndif * ndif + edif * edif).sqrt()
    }
}
