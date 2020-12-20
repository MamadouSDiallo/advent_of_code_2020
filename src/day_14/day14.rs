use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    todo,
};

fn convert_to_base2(num: usize) -> Vec<char> {
    let numf64 = num as f64;
    let size = numf64.log2() as usize;

    let num_bits = 36;
    let mut num_base2 = vec!['x'; num_bits];
    let mut num_modulo = num;
    for i in (0..(size + 1)).rev() {
        // println!("{}", i);
        if i == size && i != 0 {
            num_base2[num_bits - 1 - i] = '1';
            num_modulo %= 2usize.pow(i as u32);
        } else {
            if num_modulo < 2usize.pow(i as u32) {
                num_base2[num_bits - 1 - i] = '0';
            } else {
                if i == 0 {
                    if num_modulo == 0 {
                        num_base2[num_bits - 1] = '0';
                    } else {
                        num_base2[num_bits - 1] = '1';
                    };
                } else {
                    num_base2[num_bits - 1 - i] = '1';
                    num_modulo %= 2usize.pow(i as u32);
                }
            }
        }
    }

    num_base2
}

pub fn challenge_01() -> usize {
    let input_file = File::open("./src/day_14/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let lines = input.lines();

    let mut mem_values: HashMap<usize, usize> = HashMap::new();
    let mut mask = Vec::new();
    for line in lines {
        let row = line.unwrap();
        if row.chars().take(4).collect::<String>() == "mask".to_string() {
            mask = row
                .split("=")
                .nth(1)
                .unwrap()
                .trim()
                .chars()
                .collect::<Vec<char>>();
        } else {
            let mut mem = row.split("=");
            let mem_address = mem
                .next()
                .unwrap()
                .split("[")
                .nth(1)
                .unwrap()
                .split("]")
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let mut mem_value = mem.next().unwrap().trim().parse::<usize>().unwrap();
            let mem_value_base2 = convert_to_base2(mem_value);
            mem_value = 0;
            for (k, mut v) in mem_value_base2.iter().rev().enumerate() {
                if mask[35 - k] == '0' {
                    v = &'0';
                } else if mask[35 - k] == '1' {
                    v = &'1';
                }

                if v == &'1' {
                    mem_value += 2usize.pow(k as u32);
                }
            }

            mem_values.insert(mem_address, mem_value);
        }
    }
    let mut tot_mem_val = 0;
    for (_, v) in mem_values {
        tot_mem_val += v;
    }
    tot_mem_val
}

fn write_memory_values(
    mem_address_values: &HashMap<usize, usize>,
    mem_address: usize,
    mask: &Vec<char>,
) {
    let mem_value = 0;
    let mut mem_address_base2 = convert_to_base2(mem_address);
    for k in 0..mem_address_base2.len() {
        if mask[k] == '1' {
            mem_address_base2[k] = '1';
        } else if mask[k] == 'x' {
            mem_address_base2[k] = 'x';
        }
    }
    mem_address_values.insert(mem_address, mem_value);
}

pub fn challenge_02() {
    let input_file = File::open("./src/day_14/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let lines = input.lines();

    let mut mem_values: HashMap<usize, usize> = HashMap::new();
    let mut mask = Vec::new();
    for line in lines {
        let row = line.unwrap();
        if row.chars().take(4).collect::<String>() == "mask".to_string() {
            mask = row
                .split("=")
                .nth(1)
                .unwrap()
                .trim()
                .chars()
                .collect::<Vec<char>>();
        } else {
            let mut mem = row.split("=");
            let mem_address = mem
                .next()
                .unwrap()
                .split("[")
                .nth(1)
                .unwrap()
                .split("]")
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let mut mem_value = mem.next().unwrap().trim().parse::<usize>().unwrap();

            mem_value = 0;

            write_memory_values(&mem_values, mem_address, &mask);
        }
    }
    let mut tot_mem_val = 0;
    for (_, v) in mem_values {
        tot_mem_val += v;
    }
    tot_mem_val
}
