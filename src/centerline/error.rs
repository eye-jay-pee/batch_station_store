use super::{CenterLine, FileError, Point, Station};
use std::{/*cmp::Ordering,*/ result::Result};
#[derive(Debug)]
pub enum CenterLineError {
    CLFileError(FileError),
    StationGap(usize, Station, Station),
    StationOverlap(usize, Station, Station),
    PositionGap(usize, Point, Point),
    IncoherentRecords(usize),
}
pub type CenterLineResult<T> = Result<T, CenterLineError>;
impl CenterLine {
    pub fn validate(&self) -> CenterLineResult<()> {
        Ok(())
        //for (i, joint) in self.elements.windows(2).enumerate() {
        //    let prev = joint[0].get_end();
        //    let next = joint[1].get_start();
        //    if prev.pt != next.pt {
        //        Err(CenterLineError::PositionGap(i, prev.pt, next.pt))
        //    } else {
        //        match prev.sta.cmp(&next.sta) {
        //            Ordering::Less => Err(CenterLineError::StationOverlap(
        //                i, prev.sta, next.sta,
        //            )),
        //            Ordering::Equal => (),
        //            Ordering::Greater => {
        //                Err(CenterLineError::StationGap(i, prev.sta, next.sta))
        //            }
        //        }
        //    }
        //}
    }
}
mod convert {
    use super::{CenterLineError, FileError};
    impl From<FileError> for CenterLineError {
        fn from(err: FileError) -> Self {
            CenterLineError::CLFileError(err)
        }
    }
}
// impl fmt::Display for CenterLineError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//             Self::CLFileError(fe) => writeln!(f, "File Error: {}", fe),
//             Self::StationGap(usize,
//
//
//
//             Self::FileE(fe) => writeln!(f, "File Error: {}", fe),
//             Self::SegmentOverlap(a, b) => {
//                 writeln!(f, "Segments overlap from {} to {}", a, b)
//             }
//             Self::Disconnected(i) => {
//                 writeln!(f, "Segments {} and {} do not connect", i, i + 1)
//             }
//         }
//     }
// }
