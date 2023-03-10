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

fn move_child(parent_position: &(i32, i32), child_position: &mut (i32, i32)) {
    let mut move_x: i32 = parent_position.0 - child_position.0;
    let mut move_y: i32 = parent_position.1 - child_position.1;

    // Shorten a distance of 2 -> 1 so that child doesn't stay on top of parent
    move_x = match move_x {
        2 => 1,
        -2 => -1,
        _ => move_x,
    };

    move_y = match move_y {
        2 => 1,
        -2 => -1,
        _ => move_y,
    };

    // Change child position
    child_position.0 += move_x;
    child_position.1 += move_y;
}

fn simulate_part_1(lines: &Vec<String>, visited_positions: &mut HashSet<(i32, i32)>) {
    let char_to_direction: HashMap<char, (i32, i32)> =
        HashMap::from([('L', (-1, 0)), ('R', (1, 0)), ('U', (0, 1)), ('D', (0, -1))]);

    let mut tail_position = (0, 0);
    let mut head_position = (0, 0);

    visited_positions.insert((0, 0));

    for line in lines.iter() {
        let split: Vec<&str> = line.split(' ').collect();

        // Parse input
        let direction = char_to_direction[&split[0].to_string().chars().nth(0).unwrap()];
        let move_count: usize = split[1].parse().unwrap();

        // Loop through direction the correct amount of times 'L 5'
        for _ in 0..move_count {
            // Move the head
            head_position.0 += direction.0;
            head_position.1 += direction.1;

            // Check if the tail isn't adjacent, which means it has to move
            if !is_adjacent(head_position, tail_position) {
                // println!(
                //     "Old | Head: ({}, {}) Tail: ({}, {})",
                //     head_position.0, head_position.1, tail_position.0, tail_position.1
                // );

                move_child(&head_position, &mut tail_position);

                // Add to visited positions if necessary
                if !visited_positions.contains(&tail_position) {
                    visited_positions.insert(tail_position);
                }

                // println!(
                //     "New | Head: ({}, {}) Tail: ({}, {})",
                //     head_position.0, head_position.1, tail_position.0, tail_position.1
                // );
            }
        }
    }
}

fn part_1(lines: &Vec<String>, visited_positions: &mut HashSet<(i32, i32)>) {
    simulate_part_1(lines, visited_positions);
    println!("Part 1: {}", visited_positions.len());
}

// Used to print current state of positions array, mainly just to check the sample input, using a fixed size grid
fn print_sample_positions(positions: &Vec<(i32, i32)>) {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; 6]; 6];

    for (index, position) in positions.iter().enumerate() {
        if grid[position.1 as usize][position.0 as usize] == '.' {
            grid[position.1 as usize][position.0 as usize] = match index {
                0 => 'H',
                9 => 'T',
                _ => char::from_digit(u32::try_from(index).unwrap(), 10).unwrap(),
            }
        }
    }

    for row in (0..grid.len()).rev() {
        for column in 0..grid.len() {
            print!("{}", grid[row][column]);
        }
        println!();
    }

    println!();
}

fn simulate_part_2(lines: &Vec<String>, visited_positions: &mut HashSet<(i32, i32)>) {
    let char_to_direction: HashMap<char, (i32, i32)> =
        HashMap::from([('L', (-1, 0)), ('R', (1, 0)), ('U', (0, 1)), ('D', (0, -1))]);

    visited_positions.insert((0, 0));

    let mut positions: Vec<(i32, i32)> = vec![(0, 0); 10];

    // Iterate through each line of the input
    for line in lines.iter() {
        let split: Vec<&str> = line.split(' ').collect();

        // Parse input
        let direction = char_to_direction[&split[0].to_string().chars().nth(0).unwrap()];
        let move_count: usize = split[1].parse().unwrap();

        // Loop through the parsed direction 'move_count' times: 'L 5'
        for _ in 0..move_count {
            // println!("Old Head: ({}, {})", positions[0].0, positions[0].1);
            // Move the head
            positions[0].0 += direction.0;
            positions[0].1 += direction.1;
            // println!("New Head: ({}, {})", positions[0].0, positions[0].1);

            // Chain the movements from head to tail
            for position_index in 0..positions.len() - 1 {
                let parent_position = positions[position_index];

                if !is_adjacent(parent_position, positions[position_index + 1]) {
                    // println!(
                    //     "Before | Parent: ({}, {}), Child: ({}, {})",
                    //     parent_position.0,
                    //     parent_position.1,
                    //     positions[position_index + 1].0,
                    //     positions[position_index + 1].1
                    // );
                    move_child(&parent_position, &mut positions[position_index + 1]);
                    // println!(
                    //     "After | Parent: ({}, {}), Child: ({}, {})",
                    //     parent_position.0,
                    //     parent_position.1,
                    //     positions[position_index + 1].0,
                    //     positions[position_index + 1].1
                    // );
                }
            }

            // If the tail is in a new position, add it to the set
            if !visited_positions.contains(&positions[9]) {
                visited_positions.insert(positions[9]);
            }
        }

        // println!("{}", line);
        // print_sample_positions(&positions);
    }
}

fn part_2(lines: &Vec<String>, visited_positions: &mut HashSet<(i32, i32)>) {
    simulate_part_2(lines, visited_positions);
    println!("Part 2: {}", visited_positions.len());
}

fn main() {
    let file = File::open("./input.txt").expect("unable to open input");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|r| r.unwrap()).collect();

    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();

    part_1(&lines, &mut visited_positions);

    visited_positions.clear();

    part_2(&lines, &mut visited_positions);
}
