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

fn count_ways(nb_ones: u64) -> u64 {
    match nb_ones {
        0 => 1,
        1 => 1,
        2 => 2,
        3 => 4,
        4 => 7,
        _ => count_ways(nb_ones - 1),
    }
}

pub fn challenge_02() -> u64 {
    let input_file = File::open("./src/day_10/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let lines = input.lines();

    let mut numbers: Vec<u64> = Vec::new();

    for line in lines {
        numbers.push(line.unwrap().parse().unwrap());
    }
    numbers.sort();
    // println!("{:?}", numbers);

    let mut diff: Vec<u64> = Vec::new();
    diff.push(numbers[0]);
    for i in 0..(numbers.len() - 1) {
        diff.push(numbers[i + 1] - numbers[i]);
    }
    //diff.push(3);
    // println!("{:?}", diff);

    let mut count = 0;
    let mut nb_ones: Vec<u64> = Vec::new();
    for (k, &d) in diff.iter().enumerate() {
        if d != 1 {
            nb_ones.push(count);
            count = 0;
        } else {
            count += 1;
        }
        if k == diff.len() - 1 {
            nb_ones.push(count);
        }
    }
    // println!("{:?}", nb_ones);

    nb_ones.iter().fold(1, |acc, &x| acc * count_ways(x))
}
