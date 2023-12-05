fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    extract_data(&content);
}

fn extract_data(content: &str) {
    let split = content.split(":").next().unwrap();
    let mut winning_numbers: Vec<&str> = Vec::new();
    let mut cards = Vec::new();

    for line in split.lines() {
        let numbers = line.split_whitespace();

        let mut current_card = Vec::new();
        let mut is_card = true;
        for number in numbers {
            match number {
                "|" => {
                    cards.push(current_card.clone());
                    is_card = false;
                }

                _ => {
                    if is_card {
                        current_card.push(number);
                    } else {
                        winning_numbers.push(number);
                    }
                }
            }
        }
    }
}
