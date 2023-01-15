use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
}

fn get_overlapping_score<R: BufRead>(tasks: R, round: i32) -> i32 {
    match round {
        1 => {
            let mut score: i32 = 0;
            for line in tasks.lines() {
                let task = line.expect("Could not parse task");
                let task: Vec<&str> = task.split(",").collect();
                calculate_score_1(
                    task[0].split("-").collect(),
                    task[1].split("-").collect(),
                    &mut score,
                )
            }
            return score;
        }
        2 => {
            let mut score: i32 = 0;
            for line in tasks.lines() {
                let task = line.expect("Could not parse task");
                let task: Vec<&str> = task.split(",").collect();
                calculate_score_2(
                    task[0].split("-").collect(),
                    task[1].split("-").collect(),
                    &mut score,
                )
            }
            return score;
        }
        _ => (),
    }
    0
}
fn calculate_score_1(task_1: Vec<&str>, task_2: Vec<&str>, score: &mut i32) {
    let first_range: Vec<i32> = (task_1[0].parse::<i32>().expect("Couldn't parse number")
        ..(task_1[1].parse::<i32>().expect("Couldn't parse number")) + 1)
        .collect();
    let second_range: Vec<i32> = (task_2[0].parse::<i32>().expect("Couldn't parse number")
        ..(task_2[1].parse::<i32>().expect("Couldn't parse number")) + 1)
        .collect();
    match first_range.len() {
        v if (v < second_range.len()) => {
            let mut vec_iter = second_range.iter();

            if first_range.iter().all(|&x| vec_iter.any(|&item| item == x)) {
                *score += 1
            }
        }
        _ => {
            let mut vec_iter = first_range.iter();

            if second_range
                .iter()
                .all(|&x| vec_iter.any(|&item| item == x))
            {
                *score += 1
            }
        }
    }
}

fn calculate_score_2(task_1: Vec<&str>, task_2: Vec<&str>, score: &mut i32) {
    let first_range: Vec<i32> = (task_1[0].parse::<i32>().expect("Couldn't parse number")
        ..(task_1[1].parse::<i32>().expect("Couldn't parse number")) + 1)
        .collect();
    let second_range: Vec<i32> = (task_2[0].parse::<i32>().expect("Couldn't parse number")
        ..(task_2[1].parse::<i32>().expect("Couldn't parse number")) + 1)
        .collect();
    if first_range.iter().any(|x| second_range.contains(x)) {
        *score += 1
    }
}
#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use crate::get_overlapping_score;

    #[test]
    fn puzzle_1() {
        let file = File::open("./input/test_input.txt").expect("Could not open file");
        let reader = BufReader::new(file);
        let result = get_overlapping_score(reader, 1);
        assert_eq!(result, 2);
    }

    #[test]
    fn puzzle_2() {
        let file = File::open("./input/test_input.txt").expect("Could not open file");
        let reader = BufReader::new(file);
        let result = get_overlapping_score(reader, 2);
        assert_eq!(result, 4);
    }
}
