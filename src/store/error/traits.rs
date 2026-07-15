use super::StoreError;

mod format {
    use super::StoreError;
    use std::fmt::{Display, Formatter, Result};
    impl Display for StoreError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                Self::IO(e) => writeln!(f, "IO: {}", e),
                Self::CSV(e) => writeln!(f, "CSV: {}", e),
                Self::EOF => writeln!(f, "End of File"),
            }
        }
    }
}
mod convert {
    use super::StoreError;
    impl From<csv::Error> for StoreError {
        fn from(csv_e: csv::Error) -> Self {
            Self::CSV(csv_e)
        }
    }
    impl From<std::io::Error> for StoreError {
        fn from(io_e: std::io::Error) -> Self {
            Self::IO(io_e)
        }
    }
}
