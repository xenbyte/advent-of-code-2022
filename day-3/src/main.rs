use std::{
    char,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input/input.txt").expect("Could not fine the file");
    let reader = BufReader::new(file);
    let score = rucksack_get_sum(reader, 1);
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
        2 => (),
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

// fn rucksack_get_share_char(first_rucksack: &str, second_rucksack: &str) ->
#[cfg(test)]
mod tests {
    #[test]
    fn puzzle_1() {}
}
