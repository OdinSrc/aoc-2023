#![allow(unused_variables)]

pub fn run() {
    let data = super::load_file("./input.txt");
    let total = solve_aoc(data);
    println!("{total}");
}

pub fn solve_aoc(data: String) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_solution() {}
}
