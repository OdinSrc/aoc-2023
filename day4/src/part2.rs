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

    let total_cards = normalized_lines.len();

    // let w_counts: Vec<u32> = normalized_lines.iter().map(card_winning_count).collect();
    //
    // dbg!(count_sub_cards(&normalized_lines, 0, &normalized_lines[0]));
    // dbg!(w_counts);
    let card_instances: usize = normalized_lines
        .iter()
        .enumerate()
        .map(|(i, c)| count_sub_cards(&normalized_lines, i, c))
        // .inspect(|c| {
        //     dbg!(c);
        // })
        .sum();

    dbg!(card_instances + total_cards);
}

fn card_winning_count(card: &Card) -> usize {
    let (w_numbers, y_numbers) = card;
    y_numbers.iter().filter(|n| w_numbers.contains(n)).count()
}

fn count_sub_cards(all_cards: &Vec<Card>, index: usize, card: &Card) -> usize {
    let w_count = card_winning_count(card);
    if w_count == 0 {
        return 0;
    } else {
        let next_cards: Vec<&Card> = all_cards.iter().skip(index + 1).take(w_count).collect();
        let total_count: usize = next_cards.len()
            + next_cards
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    let next_index = index + 1 + i;
                    count_sub_cards(all_cards, next_index, c)
                })
                .sum::<usize>();
        // dbg!(index, next_cards);
        // println!("\n\n");
        return total_count;
    }
}
