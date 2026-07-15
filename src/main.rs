mod centerline;
use centerline::{CenterLine, CenterLineResult};
mod store;
use store::{Store, StoreError, StoreResult};

fn main() -> CenterLineResult<()> {
    let _cl = CenterLine::load("data/bourbon_k7_south.cl");
    Ok(())
}
