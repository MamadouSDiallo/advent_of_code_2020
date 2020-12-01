use std::io::prelude::*;
use std::{fs::File, io::BufReader};

pub fn challenge_01() -> usize {
    let input_file = File::open("./src/day_01/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let numbers: Vec<usize> = input.lines().map(|s| s.unwrap().parse().unwrap()).collect();
    //println!("Vector of numbers: {:?}", numbers);

    let mut prod = 0;
    'outer: for num1 in &numbers {
        for num2 in &numbers {
            if num1 + num2 == 2020 {
                //println!("First entry: {} and second entry: {}\n", &num1, &num2);
                prod = num1 * num2;
                break 'outer;
            }
        }
    }

    prod
}
