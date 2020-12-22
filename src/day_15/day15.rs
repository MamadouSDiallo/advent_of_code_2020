pub fn elves_numbers_game(max_steps: usize, numbers: &mut Vec<usize>) -> usize {
    let mut new_number = 0;
    let mut step = numbers.len() + 1;
    while step <= max_steps {
        let last_spoken = numbers[numbers.len() - 1];
        let mut new = true;
        for (k, &v) in numbers.iter().rev().enumerate() {
            if v == last_spoken && k > 0 {
                new_number = k;
                // numbers.push(new_number);
                new = false;
                break;
            }
        }
        if new {
            new_number = 0;
        }
        numbers.push(new_number);
        step += 1;
        if (step % 300_000) == 0 {
            println!("{:?}\n", step);
        }
    }
    numbers[numbers.len() - 1]
}

pub fn challenge_01() -> usize {
    let max_steps = 2020;
    let mut numbers: Vec<usize> = vec![8, 0, 17, 4, 1, 12];
    elves_numbers_game(max_steps, &mut numbers)
}
pub fn challenge_02() -> usize {
    let max_steps = 30_000_000;
    let mut numbers: Vec<usize> = vec![8, 0, 17, 4, 1, 12];
    elves_numbers_game(max_steps, &mut numbers)
}
