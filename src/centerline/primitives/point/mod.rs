use csv::{Reader, Writer};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::io::{Read, Write};

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
    pub fn load<R>(mut rdr: Reader<R>) -> PointResult<Self>
    where
        R: Read,
        Self: DeserializeOwned,
    {
        match rdr.deserialize::<Self>().next() {
            Some(Ok(point)) => Ok(point),
            Some(Err(error)) => Err(PointError::from(error)),
            None => Err(PointError::EOF),
        }
    }
    pub fn save<W>(&self, mut wtr: Writer<W>) -> PointResult<()>
    where
        W: Write,
    {
        wtr.serialize(self)?;
        wtr.flush()?;
        Ok(())
    }
}
impl Point {
    pub fn distance_to(&self, dest: &Self) -> f64 {
        let ndif = self.northing - dest.northing;
        let edif = self.easting - dest.easting;
        (ndif * ndif + edif * edif).sqrt()
    }
}
