use std::{
    collections::HashSet,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

fn part_1() {}

fn part_2() {}

fn main() {
    let file = File::open("./input.txt").expect("unable to open input");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|r| r.unwrap()).collect();

    let visited_positions: HashSet<(usize, usize)> = HashSet::new();
}
