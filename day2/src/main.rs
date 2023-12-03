#[derive(Debug)]
struct Game {
    id: usize,
    blue: usize,
    red: usize,
    green: usize,
}

impl Game {
    fn new() -> Self {
        Self {
            id: 0,
            blue: 0,
            red: 0,
            green: 0,
        }
    }

    fn new_round(&mut self) {
        // retain id
        self.blue = 0;
        self.red = 0;
        self.green = 0;
    }

    fn transfer_largest(&mut self, other: &mut Self) {
        if self.blue < other.blue {
            self.blue = other.blue;
        }

        if self.red < other.red {
            self.red = other.red;
        }

        if self.green < other.green {
            self.green = other.green;
        }
    }
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let mut games: Vec<Game> = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_terminator(&[';', ':'][..]).collect();

        let mut game = Game::new();
        let mut largest_combo = Game::new();
        for part in parts {
            let subparts: Vec<&str> = part.split_terminator(&[' ', ','][..]).collect();
            extract_data(&subparts, &mut game);

            largest_combo.transfer_largest(&mut game);

            game.new_round();
        }

        games.push(largest_combo);
    }

    println!("{:#?}", games);

    let sum_ids = games.iter().fold(0, |acc, game| acc + game.blue * game.red * game.green);

    println!("Sum of ids: {}", sum_ids);
}

fn extract_data(parts: &Vec<&str>, game: &mut Game) {
    let mut previous = String::new();
    for part in parts {
        match *part {
            "Game" => {
                game.id = parts.get(1).unwrap().parse::<usize>().unwrap();
            }

            "blue" => {
                game.blue += previous.parse::<usize>().unwrap();
            }

            "red" => {
                game.red += previous.parse::<usize>().unwrap();
            }

            "green" => {
                game.green += previous.parse::<usize>().unwrap();
            }

            _ => {
                previous = part.to_string();
            }
        }
    }
}
