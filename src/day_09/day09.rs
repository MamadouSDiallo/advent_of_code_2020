use std::io::prelude::*;
use std::{fs::File, io::BufReader};

fn find_first(numbers: &Vec<i64>) -> i64 {
    let mut k = 0;
    let mut l = 25;
    while k == 0 {
        k = 1;
        'outer: for i in (l - 25)..l {
            for j in (l - 25)..l {
                if numbers[l] == numbers[i] + numbers[j] {
                    k = 0;
                    break 'outer;
                }
            }
        }
        l += 1;
    }
    numbers[l - 1]
}

pub fn challenge_01() -> i64 {
    let input_file = File::open("./src/day_09/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let lines = input.lines();

    let mut numbers: Vec<i64> = Vec::new();

    for line in lines {
        numbers.push(line.unwrap().parse().unwrap());
    }

    find_first(&numbers)
}

pub fn challenge_02() -> i64 {
    let input_file = File::open("./src/day_09/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let lines = input.lines();

    let mut numbers: Vec<i64> = Vec::new();

    for line in lines {
        numbers.push(line.unwrap().parse().unwrap());
    }

    let mut result = 0;
    let first_number = find_first(&numbers);

    'outer: for i in 0..numbers.len() {
        for j in (i + 2)..(numbers.len() + 1) {
            let range = numbers.get(i..j).unwrap();
            if first_number == range.iter().sum() {
                result = range.iter().min().unwrap() + range.iter().max().unwrap();
                break 'outer;
            }
        }
    }

    result
}
