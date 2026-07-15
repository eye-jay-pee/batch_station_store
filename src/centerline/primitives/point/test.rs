#[allow(unused_imports)]
use super::Point;
use crate::Store;
use csv::{ReaderBuilder, Trim, WriterBuilder};
#[allow(unused_imports)]
use std::io::{Read, Write};

#[test]
fn save_and_load_points() {
    save_and_load(Point::new(1234.123, 0.9));
    save_and_load(Point::new(-1234.123, 12343325.0));
}

fn save_and_load(master: Point) {
    let path = "data/test";

    let mut writer = WriterBuilder::new()
        .has_headers(false)
        .from_path(path)
        .unwrap();
    master.save(&mut writer).unwrap();

    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .trim(Trim::All)
        .from_path(path)
        .unwrap();
    let copy = Point::load(&mut reader).unwrap();

    std::fs::remove_file(path).unwrap();

    assert_eq!(master, *copy);
}
