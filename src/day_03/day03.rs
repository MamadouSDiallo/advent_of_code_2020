use std::io::prelude::*;
use std::{fs::File, io::BufReader};

pub fn challenge_01() -> usize {
    let input_file = File::open("./src/day_03/input.txt").unwrap();
    let input = BufReader::new(input_file); //.lines(); //.next().  collect();
    let mut toboggan: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let nrows = toboggan.len();
    let ncols = toboggan[0].len();
    println!("{} -- {}", nrows, ncols);
    // println!("{:#?}", toboggan[1].len());
    let mut row = 0;
    let mut col = 0;
    let mut nb_trees = 0;
    while row < nrows - 1 {
        col = (col + 3) % ncols;
        row += 1;
        if toboggan[row][col] == '#' {
            nb_trees += 1;
            toboggan[row][col] = 'X';
        } else if toboggan[row][col] == '.' {
            toboggan[row][col] = 'O';
        }
    }
    // println!("{:?}", toboggan);
    nb_trees
}

pub fn challenge_02() {
    todo!()
}
