use std::{collections::HashMap, io::prelude::*};
use std::{fs::File, io::BufReader};

fn anyone_yes(questions: &String) -> usize {
    let mut questions_vec: Vec<&str> = questions.split("").collect();
    questions_vec.sort();
    questions_vec.dedup();

    questions_vec.len()
}

pub fn challenge_01() -> usize {
    let input_file = File::open("./src/day_06/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let lines = input.lines();

    let mut k = 0;
    let mut sum_yes = 0;
    let mut yes = String::new();
    let mut questions: Vec<String> = Vec::new();
    for line in lines {
        questions.push(line.unwrap());

        if questions[k] == "".to_string() {
            sum_yes += anyone_yes(&yes) - 1;
            // println!("{}", anyone_yes(&yes));
            yes = String::new();
        } else {
            yes.push_str(&questions[k]);
        }
        k += 1;
    }
    sum_yes += anyone_yes(&yes) - 1;
    sum_yes
}

fn everyone_yes(questions: &String, number_people: usize) -> usize {
    let mut questions_vec: Vec<&str> = questions.split("").collect();
    questions_vec.sort();
    let questions_vec = questions_vec
        .iter()
        .filter(|&x| x != &"")
        .collect::<Vec<&&str>>();

    let mut unique_chars = questions_vec.clone();
    unique_chars.dedup();

    let mut freq_chars: HashMap<&str, usize> = HashMap::new();
    for i in 0..unique_chars.len() {
        freq_chars.insert(unique_chars[i], 0);
    }

    for q in questions_vec {
        for c in unique_chars.clone() {
            if q == c {
                freq_chars.insert(c, freq_chars[c] + 1);
            }
        }
    }
    println!("{:?}", freq_chars);

    let mut number_everyone_yes = 0;
    for v in freq_chars.values() {
        if v == &number_people {
            number_everyone_yes += 1;
        }
    }

    number_everyone_yes
}

pub fn challenge_02() -> usize {
    let input_file = File::open("./src/day_06/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let lines = input.lines();

    let mut k = 0;
    let mut sum_yes = 0;
    let mut number_people = 0;
    let mut yes = String::new();
    let mut questions: Vec<String> = Vec::new();
    for line in lines {
        questions.push(line.unwrap());

        if questions[k] == "".to_string() {
            sum_yes += everyone_yes(&yes, number_people);
            println!("{}", sum_yes);
            yes = String::new();
            number_people = 0;
        } else {
            yes.push_str(&questions[k]);
            number_people += 1;
        }
        k += 1;
    }
    sum_yes += everyone_yes(&yes, number_people);
    sum_yes
}
