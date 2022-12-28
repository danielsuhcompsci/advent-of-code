use std::{
    collections::HashSet,
    env,
    fs::{read, File},
    hash::Hash,
    io::{BufRead, BufReader},
    string::String,
};

fn priority(x: &char) -> u32 {
    if x.is_lowercase() {
        return *x as u32 - 96;
    } else {
        return *x as u32 - 38;
    }
}

fn b_search(term: &char, vec: &[char], low: i32, high: i32) -> bool {
    // [1, 2, 3, 4, 5], [1, 2, 3, 4]
    if low <= high {
        let mid = (low + high) / 2;

        if priority(term) == priority(&vec[mid as usize]) {
            return true;
        }

        if priority(term) < priority(&vec[mid as usize]) {
            return b_search(term, vec, low, mid - 1);
        } else {
            return b_search(term, vec, mid + 1, high);
        }
    }
    return false;
}

fn print_rucksack(items: &Vec<char>, mid: &usize) {
    for i in 0..*mid {
        print!("{}", items[i]);
    }
    print!(" ");
    for i in *mid..items.len() {
        print!("{}", items[i]);
    }
    println!();
}

fn part_1(lines: &Vec<String>) {
    let mut sum: u32 = 0;

    // For each rucksack
    for line in lines {
        let mut item_types: Vec<char> = line.chars().collect();
        let mid = item_types.len() / 2;

        // Sort second compartment for binary search
        item_types[mid..].sort_by(|a, b| priority(a).cmp(&priority(b)));

        let first_compartment = &item_types[..mid];
        let second_compartment = &item_types[mid..];

        for (i, item_type) in first_compartment.iter().enumerate() {
            if b_search(
                item_type,
                second_compartment,
                0,
                (second_compartment.len() - 1).try_into().unwrap(),
            ) {
                // print_rucksack(&item_types, &mid);
                // println!(
                //     "{} Term: {} Priority: {}",
                //     i,
                //     item_type,
                //     priority(item_type)
                // );
                sum += priority(item_type);
                break;
            }
        }
    }
    println!("Part 1: {}", sum);
}

fn part_2(lines: &Vec<String>) {
    let mut sum: u32 = 0;

    for i in (0..lines.len()).step_by(3) {
        let first: Vec<char> = lines[i].chars().collect();
        let mut second: Vec<char> = lines[i + 1].chars().collect();
        let mut third: Vec<char> = lines[i + 2].chars().collect();

        second.sort_by(|a, b| priority(a).cmp(&priority(b)));
        third.sort_by(|a, b| priority(a).cmp(&priority(b)));

        for item_type in first.iter() {
            if b_search(
                item_type,
                &second,
                0,
                (second.len() - 1).try_into().unwrap(),
            ) {
                if b_search(item_type, &third, 0, (third.len() - 1).try_into().unwrap()) {
                    sum += priority(item_type);
                    break;
                }
            }
        }
    }

    println!("Part 2: {}", sum);
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let file = File::open("input.txt").expect("input not found");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().into_iter().map(|r| r.unwrap()).collect();

    part_1(&lines);
    part_2(&lines);
}
