use std::io::prelude::*;
use std::{fs::File, io::BufReader};

pub fn challenge_01() -> usize {
    let input_file = File::open("./src/day_04/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let mut toboggan: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    5
}

pub fn challenge_02() -> usize {
    todo!()
}
