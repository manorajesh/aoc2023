fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let (cards, winning_cards) = extract_data(&content);

    let mut total = 0;
    for (card, winning_card) in cards.iter().zip(winning_cards.iter()) {
        total += find_matching(card, winning_card);
    }

    println!("Total: {}", total);
}

fn extract_data(content: &str) -> (Vec<Vec<&str>>, Vec<Vec<&str>>) {
    let mut winning_numbers: Vec<Vec<&str>> = Vec::new();
    let mut cards: Vec<Vec<&str>> = Vec::new();

    for line in content.lines() {
        let split = line.split(":").nth(1).unwrap();

        let mut current_card = Vec::new();
        let mut current_winning_card = Vec::new();
        let mut is_card = true;
        for number in split.split_whitespace() {
            match number {
                "|" => {
                    cards.push(current_card.clone());
                    is_card = false;
                }

                _ => {
                    if is_card {
                        current_card.push(number);
                    } else {
                        current_winning_card.push(number);
                    }
                }
            }
        }
        winning_numbers.push(current_winning_card.clone());
    }

    (cards, winning_numbers)
}

fn find_matching(card: &Vec<&str>, winning_card: &Vec<&str>) -> usize {
    let mut count = 0;

    for number in card {
        if winning_card.contains(number) {
            if count == 0 {
                count += 1;
            } else {
                count *= 2;
            }
        }
    }

    count
}
