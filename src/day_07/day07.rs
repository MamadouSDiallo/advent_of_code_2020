use std::io::prelude::*;
use std::{fs::File, io::BufReader};

pub fn challenge_01() -> usize {
    let input_file = File::open("./src/day_07/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let lines = input.lines();

    for line in lines {
        println!("{}", line.unwrap());
    }
    7
}

pub fn challenge_02() -> usize {
    7
}
