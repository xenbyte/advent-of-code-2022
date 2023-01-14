use std::{
    char,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

fn main() {
    let file = File::open("./input/input.txt").expect("Could not fine the file");
    let reader = BufReader::new(file);
    let score = rucksack_get_sum(reader, 2);
    println!("Final Score is {}", score);
}

fn rucksack_get_sum<R: BufRead>(rucksacks: R, round: i32) -> i32 {
    match round {
        1 => {
            let mut score: i32 = 0;
            for line in rucksacks.lines() {
                let rucksack = line.expect("Could not parse rucksacks");
                let (first_rucksack, second_rucksack) = rucksack.split_at(rucksack.len() / 2);
                let mut common_str: char = '0';

                first_rucksack.chars().any(|c| {
                    let char = second_rucksack.contains(c);
                    if char == true {
                        common_str = c;
                    }
                    false
                });

                update_score(&mut score, common_str);
            }

            return score;
        }
        2 => {
            let mut score: i32 = 0;
            for (x, y, z) in rucksacks.lines().tuples() {
                println!("THIS");
                let common_str = find_common_str(
                    &mut score,
                    x.expect("Not Found"),
                    y.expect("Not Found"),
                    z.expect("Not Found"),
                );
                update_score(&mut score, common_str);

            }
            return score;
        }
        _ => (),
    }
    10
}

fn update_score(score: &mut i32, common_str: char) {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();

    for (i, c) in alphabet.iter().enumerate() {
        if common_str == *c {
            println!("Common Character: {}", common_str);
            println!("The index: {}", i as i32);
            *score += (i as i32) + 1
        }
    }
}

fn find_common_str(score: &mut i32, x: String, y: String, z: String) -> char {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();
    let mut common_char: char = '0';
    x.chars().any(|c| {
        if y.contains(c) && z.contains(c) {
            common_char = c;
        }
        false
    });
    return common_char;
}

// fn rucksack_get_share_char(first_rucksack: &str, second_rucksack: &str) ->
#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use crate::rucksack_get_sum;

    #[test]
    fn puzzle_1() {
        let file = File::open("./input/test_input.txt").expect("Could not fine the file");
        let reader = BufReader::new(file);
        let score = rucksack_get_sum(reader, 1);
        assert_eq!(score, 157);
    }
    #[test]
    fn puzzle_2() {
        let file = File::open("./input/test_input.txt").expect("Could not fine the file");
        let reader = BufReader::new(file);
        let score = rucksack_get_sum(reader, 2);
        assert_eq!(score, 70);
    }
}
