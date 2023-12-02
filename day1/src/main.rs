use std::collections::HashMap;

use regex::Regex;

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let mapping = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("0", 0),
    ]);
    let mut sum = 0;

    // 1.
    for line in content.lines() {
        // 2.
        let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
        let re_reversed = Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d)").unwrap();

        let first = re
            .find_iter(line)
            .nth(0)
            .map(|mat| mat.as_str())
            .unwrap();

        let reversed_line = line.chars().rev().collect::<String>();

        let last = re_reversed
            .find_iter(&reversed_line)
            .nth(0)
            .map(|mat| mat.as_str())
            .unwrap();

        let last = last.chars().rev().collect::<String>();

        let first_digit = mapping.get(first).unwrap() * 10;
        let last_digit = mapping.get(last.as_str()).unwrap();

        let number = first_digit + last_digit;
        println!("{line}");
        // println!("{matches:?}");
        println!("{number}\n");

        sum += number;
    }

    println!("{sum}");
}
