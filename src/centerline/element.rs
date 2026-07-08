use super::Anchor;
pub trait CenterLineElement {
    fn get_start(&self) -> Anchor;
    fn get_end(&self) -> Anchor;
    fn get_length(&self) -> f64;
}
