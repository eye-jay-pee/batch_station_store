#[derive(Debug)]
#[allow(dead_code)]
pub enum PointError {
    IO(std::io::Error),
    Parse(csv::Error),
    EOF,
}
#[allow(dead_code)]
pub type PointResult<T> = Result<T, PointError>;
mod traits {
    impl From<csv::Error> for super::PointError {
        fn from(csv_e: csv::Error) -> Self {
            Self::Parse(csv_e)
        }
    }
    impl From<std::io::Error> for super::PointError {
        fn from(io_e: std::io::Error) -> Self {
            Self::IO(io_e)
        }
    }
}
