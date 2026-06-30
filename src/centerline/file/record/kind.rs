use super::{Angle, Field, RecordResult, Station};

#[derive(Clone, Default)]
pub enum RecordKind {
    End,
    #[default]
    LinePoint(Station),
    PointOfCurve(Station),
    RadiusPoint(Angle),
    PointOfTangant(Station),
}
impl RecordKind {
    pub fn new_end() -> RecordResult<Self> {
        Ok(Self::End)
    }
    pub fn new_l(f: &Field) -> RecordResult<Self> {
        Ok(Self::LinePoint(Station::from(f.expect_num()?)))
    }
    pub fn new_pc(f: &Field) -> RecordResult<Self> {
        Ok(Self::LinePoint(Station::from(f.expect_num()?)))
    }
    pub fn new_r(f: &Field) -> RecordResult<Self> {
        Ok(Self::RadiusPoint(Angle::from(f.expect_num()?)))
    }
    pub fn new_pt(f: &Field) -> RecordResult<Self> {
        Ok(Self::PointOfTangant(Station::from(f.expect_num()?)))
    }
}
