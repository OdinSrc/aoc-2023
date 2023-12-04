type WinningNumbers = Vec<u32>;
type YourNumbers = Vec<u32>;
type Card = (WinningNumbers, YourNumbers);

pub fn run() {
    let input = include_str!("../input.txt");

    let normalized_lines: Vec<Card> = input
        .lines()
        .map(|line| {
            let (_, line_data) = line.split_once(":").unwrap();
            let (w_str, y_str) = line_data.split_once("|").unwrap();

            let w_numbers: WinningNumbers = w_str
                .split(" ")
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();

            let y_numbers: YourNumbers = y_str
                .split(" ")
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();

            (w_numbers, y_numbers)
        })
        .collect();

    let total: usize = normalized_lines
        .iter()
        .map(|(w_numbers, y_numbers)| {
            let c = y_numbers.iter().filter(|n| w_numbers.contains(n)).count();
            if c == 0 {
                return 0;
            }
            let i = 1 << (c - 1);
            i
        })
        .sum();

    println!("{total}");
}
