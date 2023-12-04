pub fn run() {
    let data = super::load_file();
    let total = solve_aoc(data);
    println!("{total}");
}

pub fn solve_aoc(data: String) -> u32 {
    data.lines()
        .map(|l| {
            let mut digit_str = String::new();

            let mut first_digit = '0';
            let mut last_digit: Option<char> = None;
            l.chars().for_each(|c| {
                if let Some(_) = c.to_digit(10) {
                    if digit_str.is_empty() {
                        first_digit = c;
                        digit_str.push(c);
                    } else {
                        last_digit = Some(c);
                    }
                }
            });

            if let Some(last_digit) = last_digit {
                digit_str.push(last_digit);
            } else {
                digit_str.push(first_digit);
            }

            digit_str.parse::<u32>().unwrap()
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let test_input = r#"test2blah7eightwo"#;
        let test_input = test_input.to_owned();

        assert_eq!(solve_aoc(test_input), 27);
    }
}
