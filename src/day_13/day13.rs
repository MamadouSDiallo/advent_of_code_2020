use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn challenge_01() {
    let input_file = File::open("./src/day_13/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let mut lines = input.lines();

    let timestamp = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    println!("{:?}", timestamp);
    let lineup = lines.next().unwrap().unwrap();
    let buses: Vec<usize> = lineup
        .split(",")
        .filter(|&c| c != "x")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut waiting_time: Vec<usize> = Vec::new();
    for i in 0..buses.len() {
        waiting_time.push(buses[i] - timestamp % buses[i]);
    }
    println!("Bus IDs: {:?}", buses);
    println!("Waiting times: {:?}", waiting_time);
}

pub fn challenge_02() {
    let input_file = File::open("./src/day_13/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let mut lines = input.lines();

    // let timestamp = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let lineup = lines.next().unwrap().unwrap();
    //    let departure_time = lineup.split("").fold(0, |acc| x+acc);

    let buses: Vec<usize> = lineup
        .split(",")
        .filter(|&c| c != "x")
        .map(|x| x.parse().unwrap())
        .collect();

    let positions = lineup.split(",").collect::<Vec<&str>>();
    let mut departures = Vec::new();
    for (k, v) in positions.iter().enumerate() {
        if v != &"x" {
            departures.push(k);
        }
    }

    let mut loop_again = true;
    let mut earliest = 0;
    while loop_again {
        earliest += buses[0];
        let mut waiting_time: Vec<usize> = Vec::new();
        for i in 0..buses.len() {
            waiting_time.push((earliest + departures[i]) % buses[i]);
        }

        // println!("{:?}", waiting_time);

        if waiting_time == vec![0;waiting_time.len()]
        {
            loop_again = false;
        }

        // println!("{:?}", earliest);

        // if earliest == 1068781 {
        //     break;
        // }
    }

    println!("{:?}", buses);
    println!("{:?}", departures);
    println!("{:?}", earliest);
}
