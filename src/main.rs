mod centerline;
use centerline::{CenterLine, CenterLineResult};

fn main() -> CenterLineResult<()> {
    let _cl = CenterLine::load("data/bourbon_k7_south.cl");
    Ok(())
}
