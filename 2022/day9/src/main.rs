use std::{
    collections::{HashMap, HashSet},
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

fn is_adjacent(head_position: (i32, i32), tail_position: (i32, i32)) -> bool {
    if head_position.0.abs_diff(tail_position.0) > 1 {
        return false;
    }

    if head_position.1.abs_diff(tail_position.1) > 1 {
        return false;
    }

    return true;
}

fn simulate(lines: &Vec<String>, visited_positions: &mut HashSet<(i32, i32)>) {
    let char_to_direction: HashMap<char, (i32, i32)> =
        HashMap::from([('L', (-1, 0)), ('R', (1, 0)), ('U', (0, 1)), ('D', (0, -1))]);

    let mut tail_position = (0, 0);
    let mut head_position = (0, 0);

    visited_positions.insert((0, 0));

    for line in lines.iter() {
        let split: Vec<&str> = line.split(' ').collect();

        let direction = char_to_direction[&split[0].to_string().chars().nth(0).unwrap()];
        let move_count: usize = split[1].parse().unwrap();

        for i in 0..move_count {
            head_position.0 += direction.0;
            head_position.1 += direction.1;

            if !is_adjacent(head_position, tail_position) {
                let move_x: i32 = head_position.0.abs_diff(tail_position.0) as i32;
                let move_y: i32 = head_position.1.abs_diff(tail_position.1) as i32;

                if move_x == 0 || move_y == 0 {
                    tail_position.0 += move_x;
                    tail_position.1 += move_y;

                    visited_positions.insert(tail_position);
                }
            }
        }
    }
}

fn part_1(lines: &Vec<String>, visited_positions: &mut HashSet<(i32, i32)>) {}

fn part_2() {}

fn main() {
    let file = File::open("./input.txt").expect("unable to open input");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|r| r.unwrap()).collect();

    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
}
