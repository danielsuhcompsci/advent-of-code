use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn is_start_of_packet(slice: &[char]) -> bool {
    assert!(
        slice.len() == 4,
        "invalid slice input len in is_start_of_packet"
    );

    let mut unique: HashSet<char> = HashSet::new();
    for character in slice {
        if !unique.contains(character) {
            unique.insert(character.clone());
        } else {
            return false;
        }
    }

    return true;
}

fn is_start_of_message(slice: &[char]) -> bool {
    assert!(
        slice.len() == 14,
        "invalid slice input len in is_start_of_message"
    );

    let mut unique: HashSet<char> = HashSet::new();
    for character in slice {
        if !unique.contains(character) {
            unique.insert(character.clone());
        } else {
            return false;
        }
    }

    return true;
}

fn part_1(datastream: &Vec<char>) {
    let mut start: usize = 0;
    while !is_start_of_packet(&datastream[start..start + 4]) && start < datastream.len() - 4 {
        start += 1;
    }
    println!("Part 1: {}", start + 4);
}

fn part_2(datastream: &Vec<char>) {
    let mut start: usize = 0;
    while !is_start_of_message(&datastream[start..start + 14]) && start < datastream.len() - 14 {
        start += 1;
    }
    println!("Part 2: {}", start + 14);
}

fn main() {
    let file = File::open("./input.txt").expect("unable to open input");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().into_iter().map(|r| r.unwrap()).collect();

    let datastream: Vec<char> = lines[0].chars().collect();

    part_1(&datastream);
    part_2(&datastream);
}
