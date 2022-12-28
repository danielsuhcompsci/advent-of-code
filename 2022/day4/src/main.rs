use std::{
    collections::HashSet,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

fn get_range(string: &str) -> (u32, u32) {
    let range: Vec<&str> = string.split("-").collect();
    return (
        range[0].parse::<u32>().unwrap(),
        range[1].parse::<u32>().unwrap(),
    );
}

fn fully_contains(a: &HashSet<u32>, b: &HashSet<u32>) -> bool {
    b.into_iter().all(|element| a.contains(element))
}

fn part_1(lines: &Vec<String>) {
    let mut count: u32 = 0;

    for line in lines {
        let elves: Vec<&str> = line.split(",").collect();
        let first_range = get_range(elves[0]);
        let second_range = get_range(elves[1]);

        if first_range.0 <= second_range.0 && first_range.1 >= second_range.1 {
            count += 1;
        } else if (second_range.0 <= first_range.0 && second_range.1 >= first_range.1) {
            count += 1;
        }
    }

    println!("Part 1: {}", count);
}

fn part_2(lines: &Vec<String>) {
    let mut count: u32 = 0;

    for line in lines {
        let elves: Vec<&str> = line.split(",").collect();
        let first_range = get_range(elves[0]);
        let second_range = get_range(elves[1]);

        if (first_range.0 <= second_range.0 && second_range.0 <= first_range.1)
            || (first_range.0 <= second_range.1 && second_range.1 <= first_range.1)
            || (second_range.0 <= first_range.0 && first_range.0 <= second_range.1)
            || (second_range.0 <= first_range.1 && first_range.1 <= second_range.1)
        {
            count += 1;
        }
    }

    println!("Part 2: {}", count);
}

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let buffer = BufReader::new(file);

    let lines: Vec<String> = buffer
        .lines()
        .into_iter()
        .map(|result| result.unwrap())
        .collect();

    part_1(&lines);
    part_2(&lines);
}
