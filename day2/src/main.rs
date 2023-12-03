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
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let mut games: Vec<usize> = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_terminator(&[';', ':'][..]).collect();

        let mut game = Game::new();
        let mut valid_game = true;
        for part in parts {
            let subparts: Vec<&str> = part.split_terminator(&[' ', ','][..]).collect();
            extract_data(&subparts, &mut game);

            if game.blue > 14 || game.red > 12 || game.green > 13 {
                valid_game = false;
            }
            game.new_round();

            println!("{:?}", subparts);
        }
        println!("");
        if valid_game {
            games.push(game.id);
        }
    }

    println!("{:#?}", games);

    let sum_ids = games.iter().fold(0, |acc, id| acc + id);

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
