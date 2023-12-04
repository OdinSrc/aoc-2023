use std::{fs::File, io::Read};

pub mod part_1;
pub mod part_2;

fn load_file() -> String {
    let mut f = File::open("./input.txt").expect("Unable to open the file");

    let mut data = String::new();
    f.read_to_string(&mut data).expect("Unable to read content");

    data
}
