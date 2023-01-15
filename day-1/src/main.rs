use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Elf {
    number: i32,
    total_calories: i32,
}

fn elves_with_most_calories(elves: &mut Vec<Elf>, count: i32) -> Vec<Elf> {
    let mut most_elves: Vec<Elf> = Vec::new();
    for _ in 0..count {
        let mut most_calories: i32 = elves[0].total_calories;
        let mut n: i32 = 0;
        for elf in elves.iter_mut() {
            if elf.total_calories > most_calories {
                most_calories = elf.total_calories;
            }
        }

        for elf in elves.iter_mut() {
            if elf.total_calories == most_calories {
                n = elf.number
            }
        }

        most_elves.push(Elf {
            number: n,
            total_calories: most_calories,
        });

        elves.retain(|x| x.number != n);
    }
    most_elves
}

fn parse_elves_calories(file: &str) -> Vec<Elf> {
    let file = File::open(file).expect("Could not find file");
    let reader = BufReader::new(file);
    let mut elves: Vec<Elf> = Vec::new();
    let mut number_of_elf: i32 = 1;
    let mut total_calories: i32 = 0;

    for line in reader.lines() {
        let calories: String = line.expect("Could not parse line");

        match calories {
            c if (c == "") => {
                elves.push(Elf {
                    number: number_of_elf,
                    total_calories,
                });
                total_calories = 0;
            }
            _ => {
                total_calories += calories.parse::<i32>().unwrap();
            }
        }
        number_of_elf += 1;
    }

    elves
}
fn main() {
    let mut elves = parse_elves_calories("./input/input.txt");
    let elf_with_most_calories = elves_with_most_calories(&mut elves, 5);
    println!("First Elf: {:?}", elf_with_most_calories);

}
#[cfg(test)]
mod tests {
    use crate::{elves_with_most_calories, parse_elves_calories};

    #[test]
    fn puzzle_1() {
        let mut elves_test = parse_elves_calories("./input/test_input.txt");
        let elf_with_most_calories = elves_with_most_calories(&mut elves_test, 1);
        assert_eq!(elf_with_most_calories[0].total_calories, 24000);
    }
    #[test]
    fn puzzle_2() {
        let mut elves_test = parse_elves_calories("./input/test_input.txt");
        let elf_with_most_calories = elves_with_most_calories(&mut elves_test, 3);
        let total_calories: i32 = elf_with_most_calories
            .iter()
            .map(|x| x.total_calories)
            .sum();
        assert_eq!(total_calories, 45000);
    }
}
