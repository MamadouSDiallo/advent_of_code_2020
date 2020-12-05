use std::io::prelude::*;
use std::{fs::File, io::BufReader};

pub fn challenge_01() -> usize {
    let input_file = File::open("./src/day_05/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let rows = input.lines();

    for row in rows {
        println!("{:?}", row);
    }
    5
}

pub fn challenge_02() -> usize {
    5
}
