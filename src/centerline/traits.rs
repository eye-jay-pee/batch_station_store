use super::Anchor;
mod cle {
    use super::Anchor;
    pub trait CenterLineElement {
        fn get_start(&self) -> Option<Anchor>;
        fn get_end(&self) -> Option<Anchor>;
        fn get_length(&self) -> f64;
    }
}
pub use cle::CenterLineElement;
