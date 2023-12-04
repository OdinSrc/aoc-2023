use std::collections::HashMap;

pub fn run() {
    let data = super::load_file();
    let total = solve_aoc(data);
    println!("{total}");
}

pub fn solve_aoc(data: String) -> u32 {
    data.lines()
        .map(|l| {
            let mut digit_words = find_digit_words(l);

            l.chars().enumerate().for_each(|(i, c)| {
                if let Some(_) = c.to_digit(10) {
                    digit_words.insert(i, c);
                }
            });

            let mut digit_str = String::new();

            let dw_min_index = digit_words.keys().min().unwrap();
            let first_letter = digit_words.get(dw_min_index).unwrap().to_owned();
            digit_str.push(first_letter);

            let dw_max_index = digit_words.keys().max();
            if digit_words.len() == 1 {
                digit_str.push(first_letter);
            } else {
                if let Some(dw_max_index) = dw_max_index {
                    let c = digit_words.get(&dw_max_index).unwrap().to_owned();
                    digit_str.push(c);
                }
            }

            digit_str.parse::<u32>().unwrap()
        })
        .sum::<u32>()
}

fn convert_to_digit(word: &str) -> Option<u8> {
    let d = match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "zero" => 0,
        _ => return None,
    };

    Some(d)
}

fn find_digit_words(line: &str) -> HashMap<usize, char> {
    let mut v: HashMap<usize, char> = HashMap::new();

    let words_to_find = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    // does not work on the edge case, like: blaheighttwo
    // let pattern = words_to_find.join("|");
    // let re = Regex::new(&pattern).unwrap();
    // for m in re.find_iter(line) {
    //     let oc = find_all_occurrences(line, m.as_str());
    //     oc.into_iter().for_each(|i| {
    //         let digit = convert_to_digit(m.as_str()).unwrap();
    //         v.insert(i, (digit + b'0') as char);
    //     });
    // }

    for w in words_to_find {
        if line.contains(w) {
            let oc = find_all_occurrences(line, w);
            oc.into_iter().for_each(|i| {
                let digit = convert_to_digit(w).unwrap();
                v.insert(i, (digit + b'0') as char);
            });
        }
    }
    v
}

fn find_all_occurrences(data: &str, word: &str) -> Vec<usize> {
    let mut indexes = Vec::new();

    let mut start = 0;

    while let Some(index) = data[start..].find(word) {
        let abs_index = start + index;
        indexes.push(abs_index);
        start = abs_index + word.len();
    }

    indexes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let test_input = r#"testeightwo"#;
        let test_input = test_input.to_owned();

        assert_eq!(solve_aoc(test_input), 82);
    }
}
