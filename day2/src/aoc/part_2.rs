#![allow(unused_variables)]

#[derive(Debug)]
pub struct GameRound {
    pub id: u32,
    pub sets: Vec<GameSet>,
}

#[derive(Debug)]
pub struct GameSet {
    pub total_blue: u32,
    pub total_red: u32,
    pub total_green: u32,
}

pub fn run() {
    let data = super::load_file("./input.txt");
    let total = solve_aoc(data);
    println!("{total}");
}

pub fn solve_aoc(data: String) -> u32 {
    let vg = parse_games(&data);

    vg.into_iter()
        .map(|gr| {
            let mut min_blue = 1;
            let mut min_red = 1;
            let mut min_green = 1;

            gr.sets.iter().for_each(|s| {
                if s.total_blue > min_blue {
                    min_blue = s.total_blue;
                }
                if s.total_red > min_red {
                    min_red = s.total_red;
                }
                if s.total_green > min_green {
                    min_green = s.total_green;
                }
            });

            min_blue * min_red * min_green
        })
        .sum()
}

fn parse_games(data: &str) -> Vec<GameRound> {
    data.lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (id_part, sets_part) = l.split_once(":").unwrap();

            let (_, game_id) = id_part.split_once(" ").unwrap();

            let games_set: Vec<GameSet> = sets_part
                .split(";")
                .map(|l| {
                    let selected_cubes: Vec<&str> = l.split(",").collect();

                    let mut total_blue = 0;
                    let mut total_red = 0;
                    let mut total_green = 0;

                    selected_cubes.iter().for_each(|cube_str| {
                        let cube_str = cube_str.trim();
                        let cube_str: Vec<&str> = cube_str.split(" ").collect();

                        match cube_str[1] {
                            "blue" => total_blue = cube_str[0].parse().unwrap(),
                            "green" => total_green = cube_str[0].parse().unwrap(),
                            "red" => total_red = cube_str[0].parse().unwrap(),
                            _ => panic!("invalid data"),
                        }
                    });

                    let game_set = GameSet {
                        total_blue,
                        total_red,
                        total_green,
                    };

                    game_set
                })
                .collect();

            GameRound {
                id: game_id.parse().unwrap(),
                sets: games_set,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use crate::aoc::{load_file, part_2::solve_aoc};

    use super::*;

    #[test]
    fn test_solution() {
        let test_data = load_file("./test.txt");
        assert!(!test_data.is_empty());

        let total = solve_aoc(test_data);
        assert_eq!(total, 2286)
    }
}
