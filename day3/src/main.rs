// use std::time::{ SystemTime, UNIX_EPOCH };

// #[derive(Debug)]
// enum Cell {
//     Number(PartNumber),
//     Empty,
//     Symbol(char),
// }

// #[derive(Debug, PartialEq, Clone, Copy)]
// struct PartNumber {
//     number: usize,
//     id: usize,
// }

// impl PartNumber {
//     fn new(number: usize, id: usize) -> Self {
//         Self { number, id }
//     }
// }

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let engine_schematic: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    println!("Part 2: {}", asterisk_symbols(&engine_schematic));
}

fn asterisk_symbols(engine_schematic: &Vec<Vec<char>>) -> usize {
    let mut sum: usize = 0;
    for row in 0..engine_schematic.len() {
        for col in 0..engine_schematic[row].len() {
            let symbol = engine_schematic[row][col];
            match symbol {
                '*' => {
                    let ratio = search_for_two_numbers(engine_schematic, row, col);

                    if let Some(ratio) = ratio {
                        sum += ratio;
                        print_color("*", 10);
                    } else {
                        print_color("*", 9);
                    }
                }
                _ => {
                    print!("{}", symbol);
                }
            }
        }
        println!();
    }
    sum
}

fn search_for_two_numbers(
    engine_schematic: &Vec<Vec<char>>,
    row: usize,
    col: usize
) -> Option<usize> {
    let mut product: Vec<usize> = Vec::with_capacity(2);

    for i in 0..3 {
        for j in 0..3 {
            let x = (row + i).saturating_sub(1);
            if let Some(number) = engine_schematic.get(x) {
                let y = (col + j).saturating_sub(1);
                if let Some(number) = number.get(y) {
                    match number {
                        '0'..='9' => {
                            let number = get_number(x, y, engine_schematic);
                            if let Some(number) = number {
                                if !product.contains(&number) {
                                    product.push(number);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    if product.len() == 2 {
        return Some(product[0] * product[1]); // gear ratio calculation
    } else {
        return None;
    }
}

fn get_number(startx: usize, starty: usize, engine_schematic: &Vec<Vec<char>>) -> Option<usize> {
    let mut current_number: String = String::new();
    let mut y = starty;

    // go to beginning of number
    loop {
        let symbol = engine_schematic[startx][y];
        match symbol {
            '0'..='9' => {
                if y <= 0 {
                    break;
                }

                y -= 1;
            }
            _ => {
                y += 1;
                break;
            }
        }
    }

    // read number
    loop {
        let symbol = engine_schematic[startx][y];
        match symbol {
            '0'..='9' => {
                current_number.push(symbol);
                if y >= engine_schematic[startx].len() - 1 {
                    break;
                }

                y += 1;
            }
            _ => {
                break;
            }
        }
    }

    if let Ok(number) = current_number.parse::<usize>() {
        return Some(number);
    }

    None
}

// fn asterisk_symbols(numbers: &Vec<Vec<Cell>>) -> usize {
//     let char_set = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
//     let mut sum: usize = 0;
//     for row in 0..numbers.len() {
//         for col in 0..numbers[row].len() {
//             match numbers[row][col] {
//                 Cell::Symbol('*') => {
//                     // sum += search_for_two_numbers(numbers, row, col).unwrap_or(0);
//                     let ratio = search_for_two_numbers(numbers, row, col);

//                     if let Some(ratio) = ratio {
//                         sum += ratio;
//                         print_color("*", 10);
//                     } else {
//                         print_color("*", 9);
//                     }
//                 }

//                 Cell::Number(num) => {
//                     print!("{}", char_set[num.number % 9]);
//                 }

//                 Cell::Empty => {
//                     print!(".");
//                 }

//                 Cell::Symbol(symbol) => {
//                     print!("{}", symbol);
//                 }
//             }
//         }
//         println!();
//     }

//     sum
// }

// fn search_for_two_numbers(numbers: &Vec<Vec<Cell>>, row: usize, col: usize) -> Option<usize> {
//     let mut adjecent_numbers: Vec<PartNumber> = Vec::with_capacity(2);
//     for i in 0..3 {
//         for j in 0..3 {
//             let x = (row + i).saturating_sub(1);
//             if let Some(number) = numbers.get(x) {
//                 let y = (col + j).saturating_sub(1);
//                 if let Some(number) = number.get(y) {
//                     match number {
//                         Cell::Number(num) => {
//                             if !adjecent_numbers.contains(num) {
//                                 adjecent_numbers.push(*num);
//                             }
//                         }
//                         _ => {}
//                     }
//                 }
//             }
//         }
//     }

//     if adjecent_numbers.len() == 2 {
//         return Some(adjecent_numbers[0].number * adjecent_numbers[1].number); // gear ratio calculation
//     }

//     None
// }

// fn populate_numbers(engine_schematic: &Vec<Vec<char>>) -> Vec<Vec<Cell>> {
//     let mut result: Vec<Vec<Cell>> = Vec::new();

//     let mut current_number: String = String::new();
//     for row in 0..engine_schematic.len() {
//         let mut row_result: Vec<Cell> = Vec::new();
//         for col in 0..engine_schematic[row].len() {
//             let symbol = engine_schematic[row][col];
//             match symbol {
//                 '0'..='9' => {
//                     current_number.push(symbol);
//                 }
//                 _ => {
//                     if !current_number.is_empty() {
//                         for _ in 0..current_number.len() {
//                             row_result.push(
//                                 Cell::Number(
//                                     PartNumber::new(
//                                         current_number.parse::<usize>().unwrap(),
//                                         current_timestamp()
//                                     )
//                                 )
//                             );
//                         }
//                         current_number = String::new();
//                     }

//                     if symbol == '.' {
//                         row_result.push(Cell::Empty);
//                     } else {
//                         row_result.push(Cell::Symbol(symbol));
//                     }
//                 }
//             }
//         }
//         result.push(row_result);
//     }

//     result
// }

fn print_color(s: &str, color: u8) {
    print!("\x1b[38;5;{}m{}\x1b[0m", color, s);
}

// fn current_timestamp() -> usize {
//     let start = SystemTime::now();
//     let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");

//     // Using as_millis() to get the timestamp in milliseconds
//     // and then converting it to usize
//     since_the_epoch.as_millis() as usize
// }
