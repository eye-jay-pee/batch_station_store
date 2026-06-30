use super::{Angle, Point, Station};
mod error;
pub use error::{RecordError, RecordResult};
mod kind;
pub use kind::RecordKind;
mod field;
use field::{Field, FieldError};
#[derive(Clone, Default)]
pub struct Record {
    pub kind: RecordKind,
    pub pos: Anchor,
}
impl Record {
    pub fn load(text: &str) -> RecordResult<Self> {
        let mut fields: Vec<Field> = Vec::new();
        for field_text in text.split(',') {
            fields.push(Field::load(field_text));
        }
        Self::new(&fields)
    }
}
impl Record {
    const FIELD_COUNT: usize = 5;
    fn new(fields: &[Field]) -> RecordResult<Self> {
        let n = fields.len();
        match n {
            ..Self::FIELD_COUNT => Err(RecordError::TooFewFields(n)),
            Self::FIELD_COUNT => {
                let mut record = Self::default();
                record.pos.easting = fields[4].expect_num()?;
                record.pos.northing = fields[3].expect_num()?;
                match fields[2].expect_text()?.as_str() {
                    "L" => record.kind = RecordKind::new_l(&fields[1])?,
                    "PC" => record.kind = RecordKind::new_pc(&fields[1])?,
                    "R" => record.kind = RecordKind::new_r(&fields[1])?,
                    "PT" => record.kind = RecordKind::new_pt(&fields[1])?,
                    idk => {
                        return Err(RecordError::UnknownKind(idk.to_string()))
                    }
                };
                record.pos.id = match fields[0].expect_uint()? {
                    0 => None,
                    n => Some(n),
                };
                Ok(record)
            }
            _ => Err(RecordError::TooManyFields(n)),
        }
    }
}
