use std::fmt;
use std::ops::Sub;
#[derive(Copy, Clone, Default)]
pub struct Point {
    pub id: Option<u32>,
    pub northing: f64,
    pub easting: f64,
}
impl Point {
    pub fn new(northing: f64, easting: f64) -> Self {
        Self {
            id: None,
            northing: northing,
            easting: easting,
        }
    }
    pub fn with_id(id: u32, northing: f64, easting: f64) -> Self {
        let pt = Self::new(northing, easting);
        pt.id = Some(id);
        pt
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(id) = self.id {
            writeln!(f, "Point {}", id)?;
        }
        writeln!(f, "Northing: {:021.10}", self.northing)?;
        writeln!(f, "Easting:  {:021.10}", self.easting)?;
        Ok(())
    }
}
impl Sub for Point {
    type Output = f64;
    fn sub(self, rhs: Point) -> Self::Output {
        let ndif = self.northing - rhs.northing;
        let edif = self.easting - rhs.easting;
        (ndif * ndif + edif * edif).sqrt()
    }
}
