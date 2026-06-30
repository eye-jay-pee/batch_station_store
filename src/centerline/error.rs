use super::file::FileError;
use std::result::Result;
#[derive(Debug)]
pub enum CenterLineError {
    CLFileError(FileError),
    StationGap(usize, Station, Station),
    StationOverlap(usize, Station, Station),
    PositionGap(usize, Position, Position),
}
pub type CenterLineResult<T> = Result<T, CLError>;
impl CenterLine {
    pub fn validate(&self) -> CenterLineResult<()> {
        for (i, joint) in self.elements.windows(2).enumerate() {
            let prev = joint[0].get_end();
            let next = joint[1].get_start();
            if prev.pt != next.pt {
                return Err(CenterLineError::Disconnected(i));
            }
            match prev.sta.cmp(&next.sta) {
                Ordering::Less => 
            }
        }
    }
}
mod convert {
    impl From<FileError> for CenterLineError {
        fn from(err: FileError) -> Self {
            CLError::File(err)
        }
    }
}
impl fmt::Display for CenterLineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileE(fe) => writeln!(f, "File Error: {}", fe),
            Self::SegmentOverlap(a, b) => {
                writeln!(f, "Segments overlap from {} to {}", a, b)
            }
            Self::Disconnected(i) => {
                writeln!(f, "Segments {} and {} do not connect", i, i + 1)
            }
        }
    }
}
