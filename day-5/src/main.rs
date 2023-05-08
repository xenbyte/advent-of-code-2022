fn main() {}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn puzzle_1_and_2() {
        let input = fs::read_to_string("./input/input_test.txt").expect("Could not open file");

        let (boxes, instructions) = input.split_once("\n\n").unwrap();
        let stacks_count: Vec<&str> = boxes.lines().rev().map(|line| line).collect();
        let max_stacks: Vec<i32> = stacks_count[0]
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let mut stacks1: Vec<Vec<char>> = vec![vec![]; *max_stacks.iter().max().unwrap() as usize];

        for line in boxes.lines().rev() {
            for (i, char) in line.chars().enumerate() {
                if char.is_alphabetic() {
                    stacks1[i / 4].push(char as char);
                }
            }
        }

        let mut stack2 = stacks1.clone();

        for line in instructions.lines() {
            let instruction: Vec<usize> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();

            for _ in 0..instruction[0] {
                let item = stacks1[instruction[1] - 1].pop().unwrap();
                stacks1[instruction[2] - 1].push(item);
            }
        }

        assert_eq!(
            "CMZ",
            stacks1.iter().filter_map(|s| s.last()).collect::<String>()
        );

        for line in instructions.lines() {
            let instruction: Vec<usize> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();

            let len = stack2[instruction[2] - 1].len() + instruction[0];
            stack2[instruction[2] - 1].resize(len, 'x');
            for i in 0..instruction[0] {
                let item = stack2[instruction[1] - 1].pop().unwrap();
                stack2[instruction[2] - 1][len - 1 - i] = item;
            }
        }
        assert_eq!(
            "MCD",
            stack2.iter().filter_map(|s| s.last()).collect::<String>()
        )
    }
}
