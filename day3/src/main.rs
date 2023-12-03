fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let engine_schematic: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let valid_part_numbers = get_valid_part_numbers(&engine_schematic);

    let sum = valid_part_numbers.iter().sum::<usize>();

    println!("Sum of valid part numbers: {}", sum);
}

fn get_valid_part_numbers(engine_schematic: &Vec<Vec<char>>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();

    let mut current_part_number: String = String::new();
    let mut adjacent_to_symbol: bool = false;
    for row in 0..engine_schematic.len() {
        for col in 0..engine_schematic[row].len() {
            let symbol = engine_schematic[row][col];
            match symbol {
                '0'..='9' => {
                    current_part_number.push(symbol);
                    if !adjacent_to_symbol {
                        adjacent_to_symbol = search_radius_for_symbol(engine_schematic, row, col);
                    }
                }

                _ => {
                    if !current_part_number.is_empty() && adjacent_to_symbol {
                        result.push(current_part_number.parse().unwrap());
                    }

                    print_part_number(current_part_number.clone(), adjacent_to_symbol);
                    print!("{}", symbol);

                    current_part_number = String::new();
                    adjacent_to_symbol = false;
                }
            }
        }
        println!();
    }

    result
}

fn search_radius_for_symbol(engine_schematic: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            let x = (row + i).saturating_sub(1);
            if let Some(symbol) = engine_schematic.get(x) {
                let y = (col + j).saturating_sub(1);
                if let Some(symbol) = symbol.get(y) {
                    match symbol {
                        '0'..='9' => {
                            continue;
                        }
                        '.' => {
                            continue;
                        }
                        _ => {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

fn print_color(s: &str, color: u8) {
    print!("\x1b[38;5;{}m{}\x1b[0m", color, s);
}

fn print_part_number(num: String, adjacent_to_symbol: bool) {
    if adjacent_to_symbol {
        print_color(&num, 10);
    } else {
        print_color(&num, 9);
    }
}
