use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").expect("Could not find file");
    let reader = BufReader::new(file);
}

fn find_total_score<R: BufRead>(scores: R, round: i32) -> i32 {
    let mut my_score: i32 = 0;
    match round {
        1 => {
            for line in scores.lines() {
                let game = line.expect("Could not parse game");
                my_score += get_score_from_round_1(
                    &game.chars().nth(0).unwrap(),
                    &game.chars().nth(2).unwrap(),
                );
            }
        }
        2 => {
            for line in scores.lines() {
                let game = line.expect("Could not parse game");
                my_score += get_score_from_round_2(
                    &game.chars().nth(0).unwrap(),
                    &game.chars().nth(2).unwrap(),
                );
            }
        }
        _ => (),
    }

    my_score
}

fn get_score_from_round_1(elf: &char, me: &char) -> i32 {
    let mut score = 0;
    match me {
        'X' => {
            score += 1;
            match elf {
                'A' => score += 3,
                'B' => (),
                'C' => score += 6,
                _ => (),
            }
        }
        'Y' => {
            score += 2;
            match elf {
                'A' => score += 6,
                'B' => score += 3,
                'C' => (),
                _ => (),
            }
        }
        'Z' => {
            score += 3;
            match elf {
                'A' => (),
                'B' => score += 6,
                'C' => score += 3,
                _ => (),
            }
        }
        _ => (),
    }
    score
}

fn get_score_from_round_2(elf: &char, me: &char) -> i32 {
    let mut score = 0;
    match me {
        'X' => {
            score += 0;
            match elf {
                'A' => score += 3,
                'B' => score += 1,
                'C' => score += 2,
                _ => (),
            }
        }
        'Y' => {
            score += 3;
            match elf {
                'A' => score += 1,
                'B' => score += 2,
                'C' => score += 3,
                _ => (),
            }
        }
        'Z' => {
            score += 6;
            match elf {
                'A' => score += 2,
                'B' => score += 3,
                'C' => score += 1,
                _ => (),
            }
        }
        _ => (),
    }
    score
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use crate::find_total_score;


    #[test]
    fn puzzle_1() {
        let file = File::open("./input/test_input.txt").expect("Could not find file");
        let reader = BufReader::new(file);
        let scores = find_total_score(reader, 1);
        
        assert_eq!(scores, 15);
    }
    #[test]
    fn puzzle_2() {
        let file = File::open("./input/test_input.txt").expect("Could not find file");
        let reader = BufReader::new(file);
        let scores = find_total_score(reader, 2);
        
        assert_eq!(scores, 12);
    }
}
