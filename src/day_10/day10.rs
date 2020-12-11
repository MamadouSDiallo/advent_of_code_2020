use std::io::prelude::*;
use std::{fs::File, io::BufReader};

pub fn challenge_01() -> usize {
    let input_file = File::open("./src/day_10/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let lines = input.lines();

    let mut numbers: Vec<i32> = Vec::new();

    for line in lines {
        numbers.push(line.unwrap().parse().unwrap());
    }
    // println!("{:?}", numbers);

    numbers.sort();

    let mut diff: Vec<i32> = Vec::new();
    diff.push(numbers[0]);
    for i in 0..(numbers.len() - 1) {
        diff.push(numbers[i + 1] - numbers[i]);
    }
    diff.push(3);

    let nb_ones = diff
        .iter()
        .filter(|&&x| x == 1)
        .collect::<Vec<&i32>>()
        .len();
    let nb_threes = diff
        .iter()
        .filter(|&&x| x == 3)
        .collect::<Vec<&i32>>()
        .len();

    // println!("{}, {}, {}", &nb_ones, &nb_threes, numbers.len());
    nb_ones * nb_threes
}

pub fn challenge_02() -> usize {
    5
}
