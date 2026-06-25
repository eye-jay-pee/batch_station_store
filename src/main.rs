mod centerline;
use centerline::{CLResult, CenterLine};

fn main() -> CLResult<()> {
    let cl = CenterLine::load("data/bourbon_k7_south.cl")?;

    Ok(())
}
