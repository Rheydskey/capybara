use std::io::Read;

use capybara_nbt::RootCompound;

#[test]
fn from_file() {
    let mut test_file = std::fs::File::open("./tests/level.dat").unwrap();

    let mut bytes = Vec::new();
    test_file.read_to_end(&mut bytes).unwrap();

    RootCompound::parse(&bytes);
}

#[test]
fn region() {
    let mut test_file = std::fs::File::open("./tests/r.-2.-1.mca").unwrap();

    let mut bytes = Vec::new();
    test_file.read_to_end(&mut bytes).unwrap();

    //    RootCompound::parse(&bytes);
}
