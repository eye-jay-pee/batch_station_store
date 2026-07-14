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
