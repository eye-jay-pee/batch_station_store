mod error;
pub use error::{FieldError, FieldResult};
pub enum Field {
    Uint(u32),
    Int(i32),
    Number(f64),
    Text(String),
}
impl Field {
    pub fn load(t: &str) -> Self {
        match t.parse::<u32>() {
            Ok(u) => Self::Uint(u),
            Err(_) => match t.parse::<i32>() {
                Ok(i) => Self::Int(i),
                Err(_) => match t.parse::<f64>() {
                    Ok(f) => Self::Number(f),
                    Err(_) => Self::Text(t.to_string()),
                },
            },
        }
    }
    pub fn expect_uint(&self) -> FieldResult<u32> {
        match self {
            Self::Uint(u) => Ok(*u),
            _ => Err(FieldError::ExpectedUint),
        }
    }
    pub fn expect_int(&self) -> FieldResult<i32> {
        match self {
            Self::Uint(u) => Ok(*u as i32),
            Self::Int(i) => Ok(*i),
            _ => Err(FieldError::ExpectedInt),
        }
    }
    pub fn expect_num(&self) -> FieldResult<f64> {
        match self {
            Self::Uint(u) => Ok(f64::from(*u)),
            Self::Int(i) => Ok(f64::from(*i)),
            Self::Number(f) => Ok(*f),
            _ => Err(FieldError::ExpectedInt),
        }
    }
    pub fn expect_text(&self) -> FieldResult<String> {
        match self {
            Self::Uint(u) => Ok(u.to_string()),
            Self::Int(i) => Ok(i.to_string()),
            Self::Number(f) => Ok(f.to_string()),
            Self::Text(t) => Ok(t.to_string()),
        }
    }
}
