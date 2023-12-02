fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;

    // 1.
    for line in content.lines() {
        let numbers: Vec<char> = line
            .chars()
            .filter(|ch| ch.is_digit(10))
            .collect();

        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();

        let calibration = String::from(*first) + &String::from(*last);

        let calibration: u32 = calibration.parse().unwrap();
        sum += calibration;
    }

    // each ch for speed?
    // let mut first = String::new();
    // let mut last = String::new();
    // for ch in content.chars() {
    //     if ch == '\n' {
    //         let calibration = String::from(first) + &String::from(last);

    //         let calibration: u32 = calibration.parse().unwrap();
    //         sum += calibration;

    //         first = String::new();
    //         last = String::new();
    //     } else if ch.is_digit(10) {
    //         if first == "" {
    //             first = ch.to_string();
    //         } else {
    //             last = ch.to_string();
    //         }
    //     }
    // }

    println!("{sum}");
}
