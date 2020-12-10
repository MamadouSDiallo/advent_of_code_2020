use std::io::prelude::*;
use std::{fs::File, io::BufReader};

fn accumulate(operation: &Vec<String>, argument: &Vec<i32>) -> (usize, i32) {
    let mut execution: Vec<bool> = vec![false; operation.len()];

    let mut accumulator = 0;
    let mut k = 0;
    while k < execution.len() && !execution[k] {
        execution[k] = true;
        match operation[k].as_str() {
            "acc" => {
                accumulator += argument[k];
                k += 1;
            }
            "jmp" => {
                k = (k as i32 + argument[k]) as usize;
            }
            "nop" => {
                k += 1;
            }
            _ => panic!("Should not happen!"),
        };
    }
    // println!("{:?}", execution);

    (k, accumulator)
}

pub fn challenge_01() -> (usize, i32) {
    let mut operation: Vec<String> = Vec::new();
    let mut argument: Vec<i32> = Vec::new();

    let input_file = File::open("./src/day_08/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let lines = input.lines();

    for line in lines {
        let mut term = line.as_ref().unwrap().split_whitespace();
        operation.push(term.next().unwrap().to_owned());
        argument.push(term.next().unwrap().parse().unwrap());
    }
    // println!("{:?}", operation);
    // println!("{:?}", argument);
    accumulate(&operation, &argument)
}

pub fn challenge_02() -> (usize, i32) {
    let mut operation: Vec<String> = Vec::new();
    let mut argument: Vec<i32> = Vec::new();

    let input_file = File::open("./src/day_08/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let lines = input.lines();

    for line in lines {
        let mut term = line.as_ref().unwrap().split_whitespace();
        operation.push(term.next().unwrap().to_owned());
        argument.push(term.next().unwrap().parse().unwrap());
    }

    let mut i = 0;
    loop {
        // println!("{}", i);
        if operation[i] == "jmp".to_string() {
            operation[i] = "nop".to_string();
        } else if operation[i] == "nop".to_string() {
            operation[i] = "jmp".to_string();
        }

        let result = accumulate(&operation, &argument);
        // println!("{:?}", result);
        if result.0 == operation.len() {
            break result;
        }

        if operation[i] == "jmp".to_string() {
            operation[i] = "nop".to_string()
        } else if operation[i] == "nop".to_string() {
            operation[i] = "jmp".to_string()
        }
        i += 1;
    }
}
