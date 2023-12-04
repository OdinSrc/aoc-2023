use std::env;

use aoc::{part_1, part_2};

mod aoc;

#[allow(dead_code)]
fn main() {
    let part_to_run = env::args().nth(1).unwrap_or("2".to_owned());
    let part_to_run = part_to_run.trim();

    if part_to_run == "1" || part_to_run == "a" {
        part_1::run();
    } else {
        part_2::run();
    }
}
