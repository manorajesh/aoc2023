#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct PlayerCard {
    card: Vec<usize>,
    winning_card: Vec<usize>,
    id: usize,
}

impl PlayerCard {
    fn new(card: Vec<usize>, winning_card: Vec<usize>, id: usize) -> Self {
        Self {
            card,
            winning_card,
            id,
        }
    }

    fn count(&self) -> usize {
        let mut count = 0;

        for number in &self.card {
            if self.winning_card.contains(number) {
                count += 1;
            }
        }

        count
    }
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let player_cards = extract_data(&content);

    let total = count_total_cards(&player_cards.clone(), player_cards);

    // println!("Card 1 matches: {}", player_cards[0].count());

    println!("Total: {}", total);
}

fn extract_data(content: &str) -> Vec<PlayerCard> {
    let mut winning_numbers: Vec<Vec<usize>> = Vec::new();
    let mut cards: Vec<Vec<usize>> = Vec::new();
    let mut player_cards: Vec<PlayerCard> = Vec::new();

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
                        current_card.push(number.parse().unwrap());
                    } else {
                        current_winning_card.push(number.parse().unwrap());
                    }
                }
            }
        }
        winning_numbers.push(current_winning_card.clone());
        player_cards.push(
            PlayerCard::new(
                current_card.clone(),
                current_winning_card.clone(),
                player_cards.len() + 1
            )
        );
    }

    player_cards
}

fn count_total_cards(all_cards: &Vec<PlayerCard>, copies: Vec<PlayerCard>) -> usize {
    let mut total_cards = 0;

    for card in &copies {
        let next_cards = all_cards
            .get(card.id..card.count() + card.id)
            .unwrap_or(&[])
            .to_vec();

        total_cards += 1;
        total_cards += count_total_cards(all_cards, next_cards);
    }

    total_cards
}
