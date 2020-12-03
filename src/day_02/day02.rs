use std::io::prelude::*;
use std::{fs::File, io::BufReader};

pub fn challenge_01() -> u32 {
    let input_file = File::open("./src/day_02/input.txt").unwrap();
    let input = BufReader::new(input_file);

    let mut valid_count = 0;
    for line in input.lines() {
        // for word in line.unwrap().split_whitespace() {
        //     println!("word '{}'", word);
        // }
        let record = line.unwrap();
        let mut iter = record.split_whitespace();
        let mut min_max = iter.next().unwrap().split("-");
        let min: u32 = min_max.next().unwrap().parse().unwrap();
        let max: u32 = min_max.next().unwrap().parse().unwrap();
        let letter = iter.next().unwrap().chars().next().unwrap();
        let password = iter.next().unwrap().to_string();
        let letter_count: u32 = password
            .chars()
            .filter_map(|c| Some((c == letter) as u32))
            .into_iter()
            .sum();

        valid_count = match (min <= letter_count) && (letter_count <= max) {
            true => valid_count + 1,
            false => valid_count + 0,
        };

        println!(
            "{:?} -- {:?} -- {:?} -- {:?} -- {:?}",
            min, max, letter, password, letter_count
        );
    }

    // println!("{:?}", codes);

    valid_count
}

pub fn challenge_02() -> u32 {
    let input_file = File::open("./src/day_02/input.txt").unwrap();
    let input = BufReader::new(input_file);

    let mut valid_count = 0;
    for line in input.lines() {
        // for word in line.unwrap().split_whitespace() {
        //     println!("word '{}'", word);
        // }
        let record = line.unwrap();
        let mut iter = record.split_whitespace();
        let mut positions = iter.next().unwrap().split("-");
        let first: usize = positions.next().unwrap().parse().unwrap();
        let second: usize = positions.next().unwrap().parse().unwrap();
        let letter = iter.next().unwrap().chars().next().unwrap();
        let password = iter.next().unwrap().to_string();

        if ((password.chars().nth(first - 1).unwrap() == letter)
            && (password.chars().nth(second - 1).unwrap() != letter))
            || ((password.chars().nth(first - 1).unwrap() != letter)
                && (password.chars().nth(second - 1).unwrap() == letter))
        {
            valid_count += 1;
        }
    }

    // println!("{:?}", codes);

    valid_count
}
