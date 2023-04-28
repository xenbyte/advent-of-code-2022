fn main() {}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn puzzle_1() {
        let input = fs::read_to_string("./input/input.txt").expect("Could not open file");

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
                    stacks1[1 / 4].push(char as char);
                }
            }
        }

        assert_eq!("2", "2");
    }
}
