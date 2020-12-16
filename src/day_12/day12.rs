use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Position {
    north: usize,
    south: usize,
    east: usize,
    west: usize,
}
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}
#[derive(Debug)]
struct Ferry {
    position: Position,
    direction: Direction,
}

impl Ferry {
    fn new() -> Self {
        Self {
            position: Position {
                north: 0,
                south: 0,
                east: 0,
                west: 0,
            },
            direction: Direction::East,
        }
    }

    fn move_ferry(&mut self, action: char, value: usize) {
        if action == 'N' {
            if self.position.south > value {
                self.position.south -= value;
            } else if self.position.south > 0 {
                self.position.north = value - self.position.south;
                self.position.south = 0;
            } else {
                self.position.north += value;
            }
        } else if action == 'S' {
            if self.position.north > value {
                self.position.north -= value;
            } else if self.position.north > 0 {
                self.position.south = value - self.position.north;
                self.position.north = 0;
            } else {
                self.position.south += value;
            }
        } else if action == 'E' {
            if self.position.west > value {
                self.position.west -= value;
            } else if self.position.west > 0 {
                self.position.east = value - self.position.west;
                self.position.west = 0;
            } else {
                self.position.east += value;
            }
        } else if action == 'W' {
            if self.position.east > value {
                self.position.east -= value;
            } else if self.position.east > 0 {
                self.position.west = value - self.position.east;
                self.position.east = 0;
            } else {
                self.position.west += value;
            }
        } else if action == 'L' {
            match self.direction {
                Direction::North => match value {
                    90 => self.direction = Direction::West,
                    180 => self.direction = Direction::South,
                    270 => self.direction = Direction::East,
                    _ => panic!("value not valid for action 'L'"),
                },
                Direction::South => match value {
                    90 => self.direction = Direction::East,
                    180 => self.direction = Direction::North,
                    270 => self.direction = Direction::West,
                    _ => panic!("value not valid for action 'L'"),
                },
                Direction::East => match value {
                    90 => self.direction = Direction::North,
                    180 => self.direction = Direction::West,
                    270 => self.direction = Direction::South,
                    _ => panic!("value not valid for action 'L'"),
                },
                Direction::West => match value {
                    90 => self.direction = Direction::South,
                    180 => self.direction = Direction::East,
                    270 => self.direction = Direction::North,
                    _ => panic!("value not valid for action 'L'"),
                },
            }
        } else if action == 'R' {
            match self.direction {
                Direction::North => match value {
                    90 => self.direction = Direction::East,
                    180 => self.direction = Direction::South,
                    270 => self.direction = Direction::West,
                    _ => panic!("value not valid for action 'L'"),
                },
                Direction::South => match value {
                    90 => self.direction = Direction::West,
                    180 => self.direction = Direction::North,
                    270 => self.direction = Direction::East,
                    _ => panic!("value not valid for action 'L'"),
                },
                Direction::East => match value {
                    90 => self.direction = Direction::South,
                    180 => self.direction = Direction::West,
                    270 => self.direction = Direction::North,
                    _ => panic!("value not valid for action 'L'"),
                },
                Direction::West => match value {
                    90 => self.direction = Direction::North,
                    180 => self.direction = Direction::East,
                    270 => self.direction = Direction::South,
                    _ => panic!("value not valid for action 'L'"),
                },
            }
        } else if action == 'F' {
            match self.direction {
                Direction::North => {
                    if self.position.south > value {
                        self.position.south -= value;
                    } else if self.position.south > 0 {
                        self.position.north = value - self.position.south;
                        self.position.south = 0;
                    } else {
                        self.position.north += value;
                    }
                }
                Direction::South => {
                    if self.position.north > value {
                        self.position.north -= value;
                    } else if self.position.north > 0 {
                        self.position.south = value - self.position.north;
                        self.position.north = 0;
                    } else {
                        self.position.south += value;
                    }
                }
                Direction::East => {
                    if self.position.west > value {
                        self.position.west -= value;
                    } else if self.position.west > 0 {
                        self.position.east = value - self.position.west;
                        self.position.west = 0;
                    } else {
                        self.position.east += value;
                    }
                }
                Direction::West => {
                    if self.position.east > value {
                        self.position.east -= value;
                    } else if self.position.east > 0 {
                        self.position.west = value - self.position.east;
                        self.position.east = 0;
                    } else {
                        self.position.west += value;
                    }
                }
            }
        } else {
            panic!("action not valid!");
        }
    }
}

pub fn challenge_01() -> usize {
    let input_file = File::open("./src/day_12/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let lines = input.lines();

    let mut ferry = Ferry::new();

    for line in lines {
        let item = line.as_ref().unwrap();
        let action = item.chars().nth(0).unwrap();
        let value = item[1..].parse::<usize>().unwrap();

        ferry.move_ferry(action, value);

        // if action == 'R' {
        //     println!("{:?}", action);
        //     println!("{:?}", value);
        // }
    }
    // println!("{:?}", ferry);
    ferry.position.east + ferry.position.west + ferry.position.north + ferry.position.south
}

#[derive(Debug)]
struct Waypoint {
    position: Position,
}

impl Waypoint {
    fn new() -> Self {
        Self {
            position: Position {
                north: 1,
                south: 0,
                east: 10,
                west: 0,
            },
        }
    }

    fn move_waypoint(&mut self, action: char, value: usize) {
        if action == 'N' {
            if self.position.south >= value {
                self.position.south -= value;
            } else if self.position.south > 0 {
                self.position.north = value - self.position.south;
                self.position.south = 0;
            } else {
                self.position.north += value;
            }
        } else if action == 'S' {
            if self.position.north >= value {
                self.position.north -= value;
            } else if self.position.north > 0 {
                self.position.south = value - self.position.north;
                self.position.north = 0;
            } else {
                self.position.south += value;
            }
        } else if action == 'E' {
            if self.position.west >= value {
                self.position.west -= value;
            } else if self.position.west > 0 {
                self.position.east = value - self.position.west;
                self.position.west = 0;
            } else {
                self.position.east += value;
            }
        } else if action == 'W' {
            if self.position.east >= value {
                self.position.east -= value;
            } else if self.position.east > 0 {
                self.position.west = value - self.position.east;
                self.position.east = 0;
            } else {
                self.position.west += value;
            }
        } else if action == 'L' {
            match value {
                90 => {
                    let temp = self.position.east;
                    self.position.east = self.position.south;
                    self.position.south = self.position.west;
                    self.position.west = self.position.north;
                    self.position.north = temp;
                }
                180 => {
                    let temp1 = self.position.east;
                    self.position.east = self.position.west;
                    self.position.west = temp1;
                    let temp2 = self.position.north;
                    self.position.north = self.position.south;
                    self.position.south = temp2;
                }
                270 => {
                    let temp = self.position.east;
                    self.position.east = self.position.north;
                    self.position.north = self.position.west;
                    self.position.west = self.position.south;
                    self.position.south = temp;
                }
                _ => {
                    panic!("Should never happen");
                }
            }
        } else if action == 'R' {
            match value {
                90 => {
                    let temp = self.position.east;
                    self.position.east = self.position.north;
                    self.position.north = self.position.west;
                    self.position.west = self.position.south;
                    self.position.south = temp;
                }
                180 => {
                    let temp1 = self.position.east;
                    self.position.east = self.position.west;
                    self.position.west = temp1;
                    let temp2 = self.position.north;
                    self.position.north = self.position.south;
                    self.position.south = temp2;
                }
                270 => {
                    let temp = self.position.east;
                    self.position.east = self.position.south;
                    self.position.south = self.position.west;
                    self.position.west = self.position.north;
                    self.position.north = temp;
                }
                _ => {
                    panic!("Should never happen");
                }
            }
        } else {
            panic!("action not valid!");
        }
    }
}

pub fn challenge_02() -> usize {
    let input_file = File::open("./src/day_12/input.txt").unwrap();
    let input = BufReader::new(input_file);
    let lines = input.lines();

    let mut waypoint = Waypoint::new();
    let mut north = 0;
    let mut south = 0;
    let mut east = 0;
    let mut west = 0;

    for line in lines {
        let item = line.as_ref().unwrap();
        let action = item.chars().nth(0).unwrap();
        let value = item[1..].parse::<usize>().unwrap();

        if action == 'N'
            || action == 'S'
            || action == 'E'
            || action == 'W'
            || action == 'L'
            || action == 'R'
        {
            waypoint.move_waypoint(action, value);
        } else if action == 'F' {
            if waypoint.position.east > 0 {
                let jump = value * waypoint.position.east;
                if west >= jump {
                    west -= jump;
                } else if west > 0 {
                    east = jump - west;
                    west = 0;
                } else {
                    east += jump;
                }
            } else {
                let jump = value * waypoint.position.west;
                if east >= jump {
                    east -= jump;
                } else if east > 0 {
                    west = jump - east;
                    east = 0;
                } else {
                    west += jump;
                }
            }
            if waypoint.position.north > 0 {
                let jump = value * waypoint.position.north;
                if south >= jump {
                    south -= jump;
                } else if south > 0 {
                    north = jump - south;
                    south = 0;
                } else {
                    north += jump;
                }
            } else {
                let jump = value * waypoint.position.south;
                if north >= jump {
                    north -= jump;
                } else if north > 0 {
                    south = jump - north;
                    north = 0;
                } else {
                    south += jump;
                }
            }
        }

        // if action == 'R' {
        //     println!("{:?}", action);
        //     println!("{:?}", value);
        // }
    }
    println!("{}, {}, {}, {}", south, north, east, west);
    south + north + east + west
}
