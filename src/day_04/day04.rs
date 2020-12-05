use std::io::prelude::*;
use std::{fs::File, io::BufReader};

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}
impl Passport {
    fn new() -> Self {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
}

pub fn challenge_01() -> usize {
    let input_file = File::open("./src/day_04/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let lines = input.lines();

    let mut p = 0;
    let mut passports: Vec<Passport> = vec![Passport::new()];
    for line in lines {
        let pairs_vec: Vec<Vec<&str>> = line
            .iter()
            .map(|s| s.split_whitespace().collect())
            .collect();
        if pairs_vec[0].len() > 0 {
            for k in 0..pairs_vec[0].len() {
                let mut iter = pairs_vec[0][k].split(|c| c == ':');
                let key = iter.next().unwrap();
                let value = iter.next().unwrap().parse().unwrap();

                match key {
                    "byr" => passports[p].byr = Some(value),
                    "iyr" => passports[p].iyr = Some(value),
                    "eyr" => passports[p].eyr = Some(value),
                    "hgt" => passports[p].hgt = Some(value),
                    "hcl" => passports[p].hcl = Some(value),
                    "ecl" => passports[p].ecl = Some(value),
                    "pid" => passports[p].pid = Some(value),
                    "cid" => passports[p].cid = Some(value),
                    _ => println!("Error"),
                }
            }
        } else {
            passports.push(Passport::new());
            p += 1;
        }
    }
    // println!("{:?}", passports);

    let mut valid_count = 0;
    for passport in passports {
        let byr = match passport.byr {
            None => 0,
            _ => 1,
        };
        let iyr = match passport.iyr {
            None => 0,
            _ => 1,
        };
        let eyr = match passport.eyr {
            None => 0,
            _ => 1,
        };
        let hgt = match passport.hgt {
            None => 0,
            _ => 1,
        };
        let hcl = match passport.hcl {
            None => 0,
            _ => 1,
        };
        let ecl = match passport.ecl {
            None => 0,
            _ => 1,
        };
        let pid = match passport.pid {
            None => 0,
            _ => 1,
        };
        if (byr + iyr + eyr + hgt + hcl + ecl + pid) > 6 {
            valid_count += 1;
        }
    }

    valid_count
}

pub fn challenge_02() -> usize {
    let input_file = File::open("./src/day_04/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let lines = input.lines();

    let mut p = 0;
    let mut passports: Vec<Passport> = vec![Passport::new()];
    for line in lines {
        let pairs_vec: Vec<Vec<&str>> = line
            .iter()
            .map(|s| s.split_whitespace().collect())
            .collect();
        if pairs_vec[0].len() > 0 {
            for k in 0..pairs_vec[0].len() {
                let mut iter = pairs_vec[0][k].split(|c| c == ':');
                let key = iter.next().unwrap();
                let value = iter.next().unwrap().parse().unwrap();

                match key {
                    "byr" => passports[p].byr = Some(value),
                    "iyr" => passports[p].iyr = Some(value),
                    "eyr" => passports[p].eyr = Some(value),
                    "hgt" => passports[p].hgt = Some(value),
                    "hcl" => passports[p].hcl = Some(value),
                    "ecl" => passports[p].ecl = Some(value),
                    "pid" => passports[p].pid = Some(value),
                    "cid" => passports[p].cid = Some(value),
                    _ => println!("Error"),
                }
            }
        } else {
            passports.push(Passport::new());
            p += 1;
        }
    }
    // println!("{:?}", passports);

    fn validate_hgt(s: String) -> usize {
        if s.ends_with("cm")
            && (s.strip_suffix("cm").unwrap().parse::<usize>().unwrap() >= 150)
            && (s.strip_suffix("cm").unwrap().parse::<usize>().unwrap() <= 193)
        {
            1
        } else if s.ends_with("in")
            && (s.strip_suffix("in").unwrap().parse::<usize>().unwrap() >= 59)
            && (s.strip_suffix("in").unwrap().parse::<usize>().unwrap() <= 76)
        {
            1
        } else {
            0
        }
    }

    fn validate_hcl(s: String) -> usize {
        if s.len() == 7 && s.starts_with("#") {
            let re = "abcdef0123456789";
            let mut valid = 1;
            let s1 = &s[1..].to_string();
            for c in s1.split("") {
                if !re.contains(c) {
                    valid = 0;
                    // break;
                }
            }
            valid
        } else {
            0
        }
    }

    let mut valid_count = 0;
    for passport in passports {
        let byr = match passport.byr {
            None => 0,
            Some(x) => match x.parse().unwrap() {
                1920..=2002 => 1,
                _ => 0,
            },
        };
        let iyr = match passport.iyr {
            None => 0,
            Some(x) => match x.parse().unwrap() {
                2010..=2020 => 1,
                _ => 0,
            },
        };
        let eyr = match passport.eyr {
            None => 0,
            Some(x) => match x.parse().unwrap() {
                2020..=2030 => 1,
                _ => 0,
            },
        };
        let hgt = match passport.hgt {
            None => 0,
            Some(x) => validate_hgt(x),
        };
        let hcl = match passport.hcl {
            None => 0,
            Some(x) => validate_hcl(x),
        };
        let ecl = match passport.ecl {
            None => 0,
            Some(x) => match &x[..] {
                "amb" => 1,
                "blu" => 1,
                "brn" => 1,
                "gry" => 1,
                "grn" => 1,
                "hzl" => 1,
                "oth" => 1,
                _ => 0,
            },
        };
        let pid = match passport.pid {
            None => 0,
            Some(x) => match x.len() {
                9 => 1,
                _ => 0,
            },
        };
        if (byr + iyr + eyr + hgt + hcl + ecl + pid) > 6 {
            valid_count += 1;
        }
    }

    valid_count
}
