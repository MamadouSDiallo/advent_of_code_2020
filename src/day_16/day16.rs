use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn challenge_01() -> usize {
    let input_file1 = File::open("./src/day_16/rules.txt").unwrap();
    let input1 = BufReader::new(input_file1);
    let lines1 = input1.lines();

    let mut left_min: Vec<usize> = Vec::new();
    let mut left_max: Vec<usize> = Vec::new();
    let mut right_min: Vec<usize> = Vec::new();
    let mut right_max: Vec<usize> = Vec::new();

    for line in lines1 {
        let mut row = line.as_ref().unwrap().split(":");

        let mut rule_values = row.nth(1).unwrap().split("or");

        let mut rule_val_left = rule_values.next().unwrap().split("-");
        let val_left_min = rule_val_left
            .next()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        let val_left_max = rule_val_left
            .next()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        left_min.push(val_left_min);
        left_max.push(val_left_max);

        let mut rule_val_right = rule_values.next().unwrap().split("-");
        let val_right_min = rule_val_right
            .next()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        let val_right_max = rule_val_right
            .next()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        right_min.push(val_right_min);
        right_max.push(val_right_max);
    }

    let input_file2 = File::open("./src/day_16/nearby_tickets.txt").unwrap();
    let input2 = BufReader::new(input_file2);
    let lines2 = input2.lines();

    let mut error_rate = Vec::new();
    for line in lines2 {
        let ticket_values = line
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();

        for val in ticket_values {
            let mut not_valid = true;
            for k in 0..left_max.len() {
                if (left_min[k] <= val && val <= left_max[k])
                    || (right_min[k] <= val && val <= right_max[k])
                {
                    not_valid = false;
                    break;
                }
            }
            if not_valid {
                error_rate.push(val);
            }
        }
    }

    error_rate.iter().sum()
}

pub fn challenge_02() -> usize {
    let input_file1 = File::open("./src/day_16/rules.txt").unwrap();
    let input1 = BufReader::new(input_file1);
    let lines1 = input1.lines();

    let mut left_min: Vec<usize> = Vec::new();
    let mut left_max: Vec<usize> = Vec::new();
    let mut right_min: Vec<usize> = Vec::new();
    let mut right_max: Vec<usize> = Vec::new();

    for line in lines1 {
        let mut row = line.as_ref().unwrap().split(":");

        let mut rule_type = row.next().unwrap().trim();
        let mut rule_values = row.next().unwrap().split("or");

        let mut rule_val_left = rule_values.next().unwrap().split("-");
        let val_left_min = rule_val_left
            .next()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        let val_left_max = rule_val_left
            .next()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        left_min.push(val_left_min);
        left_max.push(val_left_max);

        let mut rule_val_right = rule_values.next().unwrap().split("-");
        let val_right_min = rule_val_right
            .next()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        let val_right_max = rule_val_right
            .next()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        right_min.push(val_right_min);
        right_max.push(val_right_max);
    }

    let input_file2 = File::open("./src/day_16/nearby_tickets.txt").unwrap();
    let input2 = BufReader::new(input_file2);
    let lines2 = input2.lines();

    let mut valid_tickets: Vec<Vec<usize>> = Vec::new();
    for line in lines2 {
        let ticket_values = line
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();

        let mut valid = true;
        for &val in &ticket_values {
            let mut valid_val = false;
            // println!("{}", val);
            'inner: for k in 0..left_max.len() {
                if (left_min[k] <= val && val <= left_max[k])
                    || (right_min[k] <= val && val <= right_max[k])
                {
                    valid_val = true;
                    break 'inner;
                }
            }
            valid = valid && valid_val;
            if !valid {
                break;
            }
        }
        if valid {
            valid_tickets.push(ticket_values);
        }
        // println!("\n \n");
    }

    for j in 0..valid_tickets.len() {
        println!("{:?}", valid_tickets[j][1]);
        // if valid_tickets[j][1] > 69 && valid_tickets[j][1] < 86 {
        //     println!("{:?}", valid_tickets[j][1]);
        // }
    }
    let mut fields = Vec::new();
    let mut number_fields = (0..left_max.len()).collect::<Vec<usize>>();
    for k in 0..left_max.len() {
        println!("{}", k);
        'outer: for i in 0..left_max.len() {
            let mut field = true;
            for j in 0..valid_tickets.len() {
                if (valid_tickets[j][i] < left_min[k] || valid_tickets[j][i] > left_max[k])
                    && (valid_tickets[j][i] < right_min[k] || valid_tickets[j][i] > right_max[k])
                {
                    field = false;
                    println!("{} - {} - {:?}", i, j, valid_tickets[j][i]);
                    println!("Got here");
                    break;
                }
            }
            if field {
                fields.push(i);
                println!("{:?}\n", i);
                // let index = number_fields.iter().position(|&r| r == i).unwrap();
                // println!("{:?}", index);
                // number_fields.remove(index);
                // println!("{:?}", &number_fields);
                // println!("{}", k);
                break 'outer;
            }
        }
    }

    println!("{:?}", fields);

    valid_tickets.len()
}
