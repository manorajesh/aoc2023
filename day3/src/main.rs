// use std::collections::HashMap;

// #[derive(Debug, Clone, Copy)]
// struct Part {
//     symbol: char,
//     x: usize,
//     y: usize,
// }

// impl Part {
//     fn new(symbol: char, x: usize, y: usize) -> Self {
//         Self { symbol, x, y }
//     }
// }

// impl PartialEq for Part {
//     fn eq(&self, other: &Self) -> bool {
//         self.symbol == other.symbol
//     }
// }

// impl Eq for Part {}

// impl std::hash::Hash for Part {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.symbol.hash(state);
//     }
// }

#[derive(Debug)]
enum Cell {
    Number(usize),
    Empty,
    Symbol(char),
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let engine_schematic: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let numbers = populate_numbers(&engine_schematic);

    let sum = asterisk_symbols(&numbers);

    println!("Sum of asterisk symbols: {}", sum);

    // for row in numbers {
    //     for col in row {
    //         match col {
    //             Cell::Number(num) => {
    //                 print!("{}", char_set[num % 9]);
    //             }
    //             Cell::Empty => {
    //                 print!(".");
    //             }
    //             Cell::Symbol(symbol) => {
    //                 print!("{}", symbol);
    //             }
    //         }
    //     }
    //     println!();
    // }
}

fn asterisk_symbols(numbers: &Vec<Vec<Cell>>) -> usize {
    let char_set = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
    let mut sum: usize = 0;
    for row in 0..numbers.len() {
        for col in 0..numbers[row].len() {
            match numbers[row][col] {
                Cell::Symbol('*') => {
                    // sum += search_for_two_numbers(numbers, row, col).unwrap_or(0);
                    let ratio = search_for_two_numbers(numbers, row, col);

                    if let Some(ratio) = ratio {
                        sum += ratio;
                        print_color("*", 10);
                    } else {
                        print_color("*", 9);
                    }
                }

                Cell::Number(num) => {
                    print!("{}", char_set[num % 9]);
                }

                _ => {
                    // grey
                    print_color(".", 0);
                }
            }
        }
        println!();
    }

    sum
}

fn search_for_two_numbers(numbers: &Vec<Vec<Cell>>, row: usize, col: usize) -> Option<usize> {
    let mut adjecent_numbers: Vec<usize> = Vec::with_capacity(2);
    for i in 0..3 {
        for j in 0..3 {
            let x = (row + i).saturating_sub(1);
            if let Some(number) = numbers.get(x) {
                let y = (col + j).saturating_sub(1);
                if let Some(number) = number.get(y) {
                    match number {
                        Cell::Number(num) => {
                            if !adjecent_numbers.contains(num) {
                                adjecent_numbers.push(*num);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    if adjecent_numbers.len() == 2 {
        return Some(adjecent_numbers[0] * adjecent_numbers[1]); // gear ratio calculation
    }

    None
}

fn populate_numbers(engine_schematic: &Vec<Vec<char>>) -> Vec<Vec<Cell>> {
    let mut result: Vec<Vec<Cell>> = Vec::new();

    let mut current_number: String = String::new();
    for row in 0..engine_schematic.len() {
        let mut row_result: Vec<Cell> = Vec::new();
        for col in 0..engine_schematic[row].len() {
            let symbol = engine_schematic[row][col];
            match symbol {
                '0'..='9' => {
                    current_number.push(symbol);
                }
                _ => {
                    if !current_number.is_empty() {
                        for _ in 0..current_number.len() {
                            row_result.push(Cell::Number(current_number.parse::<usize>().unwrap()));
                        }
                        current_number = String::new();
                    }

                    if symbol == '.' {
                        row_result.push(Cell::Empty);
                    } else {
                        row_result.push(Cell::Symbol(symbol));
                    }
                }
            }
        }
        result.push(row_result);
    }

    result
}

// fn get_valid_part_numbers(engine_schematic: &Vec<Vec<char>>) -> HashMap<usize, Part> {
//     let mut result: HashMap<usize, Part> = HashMap::new();

//     let mut current_part_number: String = String::new();
//     let mut adjacent_symbol: Option<Part> = None;
//     for row in 0..engine_schematic.len() {
//         for col in 0..engine_schematic[row].len() {
//             let symbol = engine_schematic[row][col];
//             match symbol {
//                 '0'..='9' => {
//                     current_part_number.push(symbol);
//                     if adjacent_symbol.is_none() {
//                         adjacent_symbol = search_radius_for_symbol(engine_schematic, row, col);
//                     }
//                 }

//                 _ => {
//                     if !current_part_number.is_empty() && adjacent_symbol.is_some() {
//                         let part_number = current_part_number.parse::<usize>().unwrap();
//                         result.insert(part_number, adjacent_symbol.unwrap());
//                     }

//                     print_part_number(current_part_number.clone(), adjacent_symbol.is_some());
//                     print!("{}", symbol);

//                     current_part_number = String::new();
//                     adjacent_symbol = None;
//                 }
//             }
//         }
//         println!();
//     }

//     result
// }

// fn search_radius_for_symbol(
//     engine_schematic: &Vec<Vec<char>>,
//     row: usize,
//     col: usize
// ) -> Option<Part> {
//     for i in 0..3 {
//         for j in 0..3 {
//             let x = (row + i).saturating_sub(1);
//             if let Some(symbol) = engine_schematic.get(x) {
//                 let y = (col + j).saturating_sub(1);
//                 if let Some(symbol) = symbol.get(y) {
//                     match symbol {
//                         '0'..='9' => {
//                             continue;
//                         }
//                         '.' => {
//                             continue;
//                         }
//                         _ => {
//                             return Some(Part::new(*symbol, x, y));
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     None
// }

fn print_color(s: &str, color: u8) {
    print!("\x1b[38;5;{}m{}\x1b[0m", color, s);
}

// fn print_part_number(num: String, adjacent_to_symbol: bool) {
//     if adjacent_to_symbol {
//         print_color(&num, 10);
//     } else {
//         print_color(&num, 9);
//     }
// }

// fn filter_shared_values(valid_part_numbers: HashMap<usize, Part>) -> HashMap<usize, Part> {
//     let mut value_counts: HashMap<Part, usize> = HashMap::new();

//     // Count the occurrences of each Part
//     for part in valid_part_numbers.values() {
//         *value_counts.entry(*part).or_insert(0) += 1;
//     }

//     // Filter the valid_part_numbers to include only those with shared values
//     valid_part_numbers
//         .into_iter()
//         .filter(
//             |&(_, part)|
//                 value_counts.get(&part).map_or(false, |&count| count > 1) && part.symbol == '*'
//         )
//         .collect()
// }
