#[allow(unused_imports)]
use advent2020::{day_01, day_02};

fn main() {
    println!("Advent Code 2020");

    // println!("\nDay 1 challenges");
    // println!(
    //     "The product of the two entries summing to 2020 is equal to: {}",
    //     day_01::challenge_01()
    // );
    // println!(
    //     "The product of the three entries summing to 2020 is equal to: {}",
    //     day_01::challenge_02()
    // );

    println!("\nDay 2 challenges");
    println!(
        "Number of valid passwords based on old policy: {}",
        day_02::challenge_01()
    );
    println!(
        "Number of passwords based on the new policy: {}",
        day_02::challenge_02()
    );
}
