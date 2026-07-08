use super::{Angle, Field, RecordResult, Station};

#[derive(Debug, Clone)]
pub enum RecordKind {
    End,
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
        Ok(Self::RadiusPoint(Angle::decimal_degrees(f.expect_num()?)))
    }
    pub fn new_pt(f: &Field) -> RecordResult<Self> {
        Ok(Self::PointOfTangant(Station::from(f.expect_num()?)))
    }
}
impl Default for RecordKind {
    fn default() -> Self {
        Self::LinePoint(Station::from(0.0))
    }
}
