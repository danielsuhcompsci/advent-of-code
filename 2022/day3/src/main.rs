use std::{
    fs::{read, File},
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

fn b_search<T: std::cmp::PartialOrd>(term: &T, vec: &Vec<T>, low: usize, high: usize) -> bool {
    // [1, 2, 3, 4, 5], [1, 2, 3, 4]
    if low < high {
        let mid = high / 2;
        if term < &vec[mid] {
            return b_search(term, vec, low, mid);
        } else if term > &vec[mid] {
            return b_search(term, vec, mid + 1, high);
        } else {
            return true;
        }
    }
    return false;
}

fn main() {
    let file = File::open("input.txt").expect("input not found");
    let reader = BufReader::new(file);

    let mut i = 0;

    let mut sum: u32 = 0;

    for line in reader.lines() {
        let mut chars: Vec<char> = line.unwrap().chars().collect();
        let mid = chars.len() / 2;

        chars[mid..].sort_by(|a, b| priority(a).cmp(&priority(b)));
        // chars[mid..].sort();

        if i < 2 {
            println!("{:?} \n {} \n", chars, chars[mid]);
            i += 1;
        }

        let mut i = 0;
        while i < mid && !b_search::<char>(&chars[i], &chars, mid, chars.len() - 1) {
            i += 1;
        }

        sum += priority(&chars[i]);
    }

    println!("{}", sum);
}
