use super::Point;
mod format {
    use super::Point;
    use std::fmt::{Display, Formatter, Result};
    impl Display for Point {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            writeln!(f, "Northing: {:021.10}", self.northing)?;
            writeln!(f, "Easting:  {:021.10}", self.easting)?;
            Ok(())
        }
    }
}
mod store {
    use super::Point;
    use crate::{Store, StoreError, StoreResult};
    use csv::{Reader, Writer};
    use std::io::{Read, Write};

    impl Store for Point {
        fn load<R: Read>(rdr: &mut Reader<R>) -> StoreResult<Box<Self>> {
            match rdr.deserialize::<Self>().next() {
                Some(Ok(point)) => Ok(Box::new(point)),
                Some(Err(error)) => Err(StoreError::CSV(error)),
                None => Err(StoreError::EOF),
            }
        }
        fn save<W: Write>(&self, wtr: &mut Writer<W>) -> StoreResult<()> {
            wtr.serialize(self)?;
            wtr.flush()?;
            Ok(())
        }
    }
}
