use std::io::prelude::*;
use std::{fs::File, io::BufReader};

fn half_region(letter: char, range: (usize, usize)) -> Option<(usize, usize)> {
    match letter {
        'F' | 'L' => Some((range.0, (range.0 + range.1 - 1) / 2)),
        'B' | 'R' => Some(((range.0 + range.1 + 1) / 2, range.1)),
        _ => None,
    }
}
pub fn challenge_01() -> usize {
    let input_file = File::open("./src/day_05/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let rows = input.lines();

    let mut max_seat_id = 0;
    for row in rows {
        // println!("{:?}", row);
        let mut range_fb = (0, 127);
        let letters: Vec<_> = row.unwrap().chars().collect();
        for i in 0..7 {
            let letter = letters[i];
            // println!("{:?}", letter);
            range_fb = half_region(letter, range_fb).unwrap();
        }
        // println!("{:?}", range_fb);

        let mut range_lr = (0, 7);
        for i in 7..=9 {
            let letter = letters[i];
            // println!("{:?}", letter);
            range_lr = half_region(letter, range_lr).unwrap();
        }
        // println!("{:?}", range_lr);

        let seat_id = range_fb.0 * 8 + range_lr.0;

        if max_seat_id < seat_id {
            max_seat_id = seat_id;
        }
    }
    max_seat_id
}

pub fn challenge_02() -> usize {
    let input_file = File::open("./src/day_05/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let rows = input.lines();

    let mut seat_id: Vec<usize> = Vec::new();
    for row in rows {
        // println!("{}", k);
        let mut range_fb = (0, 127);
        let letters: Vec<_> = row.unwrap().chars().collect();
        for i in 0..7 {
            let letter = letters[i];
            range_fb = half_region(letter, range_fb).unwrap();
        }

        let mut range_lr = (0, 7);
        for i in 7..=9 {
            let letter = letters[i];
            range_lr = half_region(letter, range_lr).unwrap();
        }

        seat_id.push(range_fb.0 * 8 + range_lr.0);
    }
    seat_id.sort();

    let mut k = 0;
    let my_seat = loop {
        k += 1;
        if seat_id[k + 1] - seat_id[k] > 1 {
            break seat_id[k] + 1;
        }
    };
    my_seat
}
