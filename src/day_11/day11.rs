use std::io::prelude::*;
use std::{fs::File, io::BufReader};

// fn count_ocuppied(seat_layout: Vec<Vec<&str>>) -> usize {
//     seat_layout.iter().filter(|&&x| x.eq("#")).collect().len()
// }

fn seating(seat_layout: &Vec<String>, row: usize, col: usize) -> (String, bool) {
    let mut seat = seat_layout[row].chars().nth(col).unwrap();

    let nrow = seat_layout.len();
    let mut min_row = row;
    let mut max_row = row;
    if row > 0 && row < nrow - 1 {
        min_row = row - 1;
        max_row = row + 1;
    } else if row == 0 {
        max_row = row + 1;
    } else if row == nrow - 1 {
        min_row = row - 1;
    }

    let ncol = seat_layout[0].chars().count();
    let mut min_col = col;
    let mut max_col = col;
    if col > 0 && col < ncol - 1 {
        min_col = col - 1;
        max_col = col + 1;
    } else if col == 0 {
        max_col = col + 1;
    } else if col == ncol - 1 {
        min_col = col - 1;
    }

    let mut change = false;
    match seat {
        'L' => {
            change = true;
            for i in min_row..(max_row + 1) {
                for j in min_col..(max_col + 1) {
                    if seat_layout[i].chars().nth(j).unwrap() == '#' {
                        change = false;
                        break;
                    }
                }
            }
            if change {
                seat = '#'
            }
        }
        '#' => {
            let mut nb_occupied = 0;
            for i in min_row..(max_row + 1) {
                for j in min_col..(max_col + 1) {
                    if seat_layout[i].chars().nth(j).unwrap() == '#' {
                        nb_occupied += 1;
                    }
                }
            }
            if nb_occupied > 4 {
                seat = 'L';
                change = true;
            }
        }
        _ => change = false,
    };

    (seat.to_string(), change)
}

pub fn challenge_01<'a>() -> usize {
    let input_file = File::open("./src/day_11/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let lines = input.lines();

    let mut seat_layout: Vec<String> = Vec::new();

    for line in lines {
        let row = line.unwrap();
        seat_layout.push(row);
    }
    // println!("{:?}", seat_layout);

    let vec_length = seat_layout.len();
    let row_length = seat_layout[0].chars().count();
    let mut no_change = false;

    while !no_change {
        no_change = true;
        let mut temp_layout: Vec<String> = vec!["".to_string(); vec_length];
        for i in 0..vec_length {
            let mut seats: String = String::new();
            for j in 0..row_length {
                let result = seating(&seat_layout, i, j);
                seats.push_str(&result.0);
                no_change = no_change && !result.1;
            }
            temp_layout[i] = seats;
        }
        seat_layout = temp_layout;
    }
    // println!("{:?}, {:?}", seat_layout, no_change);

    let mut nb_occupied_seats = 0;
    for i in 0..vec_length {
        nb_occupied_seats += seat_layout[i]
            .split("")
            .collect::<Vec<&str>>()
            .iter()
            .filter(|&s| s.eq(&"#"))
            .collect::<Vec<&&str>>()
            .len();
    }

    // println!("{}", nb_occupied_seats);
    nb_occupied_seats
}

fn update_empty_seat(seat_layout: &Vec<String>, row: usize, col: usize) -> (String, bool) {
    let nrow = seat_layout.len();
    let ncol = seat_layout[0].chars().count();

    let mut change = true;

    let mut i = row;
    while change {
        if seat_layout[i].chars().nth(col).unwrap() == '#' {
            change = false;
            break;
        } else if i == 0 || seat_layout[i].chars().nth(col).unwrap() == 'L' && i != row {
            break;
        }
        i -= 1;
    }
    let mut i = row;
    while change {
        if seat_layout[i].chars().nth(col).unwrap() == '#' {
            change = false;
            break;
        } else if i == nrow - 1 || seat_layout[i].chars().nth(col).unwrap() == 'L' && i != row {
            break;
        }
        i += 1;
    }
    let mut j = col;
    while change {
        if seat_layout[row].chars().nth(j).unwrap() == '#' {
            change = false;
            break;
        } else if j == 0 || seat_layout[row].chars().nth(j).unwrap() == 'L' && j != col {
            break;
        }
        j -= 1;
    }
    let mut j = col;
    while change {
        if seat_layout[row].chars().nth(j).unwrap() == '#' {
            change = false;
            break;
        } else if j == ncol - 1 || seat_layout[row].chars().nth(j).unwrap() == 'L' && j != col {
            break;
        }
        j += 1;
    }
    let mut i = row;
    let mut j = col;
    while change {
        if seat_layout[i].chars().nth(j).unwrap() == '#' {
            change = false;
            break;
        } else if i == 0
            || j == 0
            || seat_layout[i].chars().nth(j).unwrap() == 'L' && i != row && j != col
        {
            break;
        }
        i -= 1;
        j -= 1;
    }
    let mut i = row;
    let mut j = col;
    while change {
        if seat_layout[i].chars().nth(j).unwrap() == '#' {
            change = false;
            break;
        } else if i == nrow - 1
            || j == 0
            || seat_layout[i].chars().nth(j).unwrap() == 'L' && i != row && j != col
        {
            break;
        }
        i += 1;
        j -= 1;
    }
    let mut i = row;
    let mut j = col;
    while change {
        if seat_layout[i].chars().nth(j).unwrap() == '#' {
            change = false;
            break;
        } else if i == 0
            || j == ncol - 1
            || seat_layout[i].chars().nth(j).unwrap() == 'L' && i != row && j != col
        {
            break;
        }
        i -= 1;
        j += 1;
    }
    let mut i = row;
    let mut j = col;
    while change {
        if seat_layout[i].chars().nth(j).unwrap() == '#' {
            change = false;
            break;
        } else if i == nrow - 1
            || j == ncol - 1
            || seat_layout[i].chars().nth(j).unwrap() == 'L' && i != row && j != col
        {
            break;
        }
        i += 1;
        j += 1;
    }

    if change {
        ("#".to_string(), true)
    } else {
        ("L".to_string(), false)
    }
}

fn update_occupied_seat(seat_layout: &Vec<String>, row: usize, col: usize) -> (String, bool) {
    let nrow = seat_layout.len();
    let ncol = seat_layout[0].chars().count();

    let mut nb_occupied = 0;

    let mut i = row;
    loop {
        if seat_layout[i].chars().nth(col).unwrap() == '#' && i != row {
            nb_occupied += 1;
            break;
        } else if i == 0 || seat_layout[i].chars().nth(col).unwrap() == 'L' {
            break;
        }
        i -= 1;
    }
    let mut i = row;
    loop {
        if seat_layout[i].chars().nth(col).unwrap() == '#' && i != row {
            nb_occupied += 1;
            break;
        } else if i == nrow - 1 || seat_layout[i].chars().nth(col).unwrap() == 'L' {
            break;
        }
        i += 1;
    }
    let mut j = col;
    loop {
        if seat_layout[row].chars().nth(j).unwrap() == '#' && j != col {
            nb_occupied += 1;
            break;
        } else if j == 0 || seat_layout[row].chars().nth(j).unwrap() == 'L' {
            break;
        }
        j -= 1;
    }
    let mut j = col;
    loop {
        if seat_layout[row].chars().nth(j).unwrap() == '#' && j != col {
            nb_occupied += 1;
            break;
        } else if j == ncol - 1 || seat_layout[row].chars().nth(j).unwrap() == 'L' {
            break;
        }
        j += 1;
    }
    let mut i = row;
    let mut j = col;
    loop {
        if seat_layout[i].chars().nth(j).unwrap() == '#' && i != row && j != col {
            nb_occupied += 1;
            break;
        } else if i == 0 || j == 0 || seat_layout[i].chars().nth(j).unwrap() == 'L' {
            break;
        }
        i -= 1;
        j -= 1;
    }
    let mut i = row;
    let mut j = col;
    loop {
        if seat_layout[i].chars().nth(j).unwrap() == '#' && i != row && j != col {
            nb_occupied += 1;
            break;
        } else if i == nrow - 1 || j == 0 || seat_layout[i].chars().nth(j).unwrap() == 'L' {
            break;
        }
        i += 1;
        j -= 1;
    }
    let mut i = row;
    let mut j = col;
    loop {
        if seat_layout[i].chars().nth(j).unwrap() == '#' && i != row && j != col {
            nb_occupied += 1;
            break;
        } else if i == 0 || j == ncol - 1 || seat_layout[i].chars().nth(j).unwrap() == 'L' {
            break;
        }
        i -= 1;
        j += 1;
    }
    let mut i = row;
    let mut j = col;
    loop {
        if seat_layout[i].chars().nth(j).unwrap() == '#' && i != row && j != col {
            nb_occupied += 1;
            break;
        } else if i == nrow - 1 || j == ncol - 1 || seat_layout[i].chars().nth(j).unwrap() == 'L' {
            break;
        }
        i += 1;
        j += 1;
    }

    if nb_occupied >= 5 {
        ("L".to_string(), true)
    } else {
        ("#".to_string(), false)
    }
}

fn new_seating(seat_layout: &Vec<String>, row: usize, col: usize) -> (String, bool) {
    let seat = seat_layout[row].chars().nth(col).unwrap();

    let result = match seat {
        'L' => update_empty_seat(seat_layout, row, col),
        '#' => update_occupied_seat(seat_layout, row, col),
        _ => (seat.to_string(), false),
    };

    result
}

pub fn challenge_02() -> usize {
    let input_file = File::open("./src/day_11/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let lines = input.lines();

    let mut seat_layout: Vec<String> = Vec::new();

    for line in lines {
        let row = line.unwrap();
        seat_layout.push(row);
    }
    // println!("{:?}", seat_layout);

    let vec_length = seat_layout.len();
    let row_length = seat_layout[0].chars().count();
    let mut no_change = false;

    while !no_change {
        no_change = true;
        let mut temp_layout: Vec<String> = vec!["".to_string(); vec_length];
        for i in 0..vec_length {
            let mut seats: String = String::new();
            for j in 0..row_length {
                let result = new_seating(&seat_layout, i, j);
                seats.push_str(&result.0);
                no_change = no_change && !result.1;
            }
            temp_layout[i] = seats;
        }
        seat_layout = temp_layout;
    }
    // println!("{:?}, {:?}", seat_layout, no_change);

    let mut nb_occupied_seats = 0;
    for i in 0..vec_length {
        nb_occupied_seats += seat_layout[i]
            .split("")
            .collect::<Vec<&str>>()
            .iter()
            .filter(|&s| s.eq(&"#"))
            .collect::<Vec<&&str>>()
            .len();
    }

    // println!("{}", nb_occupied_seats);
    nb_occupied_seats
}
