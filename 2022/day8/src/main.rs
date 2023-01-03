use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type TreeGrid = Vec<Vec<u8>>;

fn create_tree_grid(lines: &Vec<String>, tree_grid: &mut TreeGrid) {
    for (row, line) in lines.iter().enumerate() {
        for (column, character) in line.chars().enumerate() {
            tree_grid[row][column] = character as u8;
        }
    }
}

fn check_visible_tree(tree_grid: &TreeGrid, row: usize, column: usize) -> bool {
    let offsets: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // (row, column)

    // For each orthogonal offset (up, down, left, right)
    for (row_offset, column_offset) in offsets {
        let mut adjacent_row = usize::checked_add_signed(row, row_offset).unwrap();
        let mut adjacent_column = usize::checked_add_signed(column, column_offset).unwrap();

        // println!("Current: ({}, {})", row, column);

        // while the adjacent tree position is in bounds
        loop {
            // if the adjacent tree is smaller than the current tree, then move on to the next adjacent tree
            if tree_grid[adjacent_row][adjacent_column] < tree_grid[row][column] {
                // If only the row changes
                if column_offset == 0 {
                    match usize::checked_add_signed(adjacent_row, row_offset) {
                        Some(row) => {
                            if row < tree_grid.len() {
                                adjacent_row = row;
                            } else {
                                // Made it to the upper bound
                                return true;
                            }
                        }
                        None => {
                            // Made it to the lower bound
                            return true;
                        }
                    }

                // If only the column changes
                } else {
                    match usize::checked_add_signed(adjacent_column, column_offset) {
                        Some(column) => {
                            if column < tree_grid[0].len() {
                                adjacent_column = column;
                            } else {
                                // Made it to the upper bound
                                return true;
                            }
                        }
                        None => {
                            // Made it to the lower bound
                            return true;
                        }
                    }
                }
            // else if the adjacent tree is the same height or taller, move on to the next offset
            } else {
                break;
            }
        }
    }

    return false;
}

fn count_visible_trees(tree_grid: &TreeGrid) -> usize {
    let mut visible_count = (tree_grid.len() * 2 + tree_grid[0].len() * 2) - 4;

    for row in 1..tree_grid.len() - 1 {
        for column in 1..tree_grid[0].len() - 1 {
            if check_visible_tree(tree_grid, row, column) {
                visible_count += 1;
            }
        }
    }

    return visible_count;
}

fn part_1(tree_grid: &TreeGrid) {
    println!("Part 1: {}", count_visible_trees(tree_grid));
}

fn get_scenic_score(tree_grid: &TreeGrid, row: usize, column: usize) -> usize {
    let offsets: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // (row, column)
    let mut distances: [usize; 4] = [0, 0, 0, 0];

    let height = tree_grid.len();
    let width = tree_grid[0].len();

    // For each orthogonal offset (up, down, left, right)
    for (i, (row_offset, column_offset)) in offsets.iter().enumerate() {
        let mut adjacent_row = row;
        let mut adjacent_column = column;

        // loop while the adjacent position is in bounds, and while the adjacent tree is smaller, then calculate distance and break the loop
        loop {
            // If only the row changes
            if *column_offset == 0 {
                match usize::checked_add_signed(adjacent_row, *row_offset) {
                    Some(some_row) => {
                        if some_row < height {
                            adjacent_row = some_row;
                        } else {
                            // Made it to the upper bound
                            distances[i] = height - 1 - row;
                            break;
                        }
                    }
                    None => {
                        // Made it to the lower bound
                        distances[i] = row;
                        break;
                    }
                }

                // if the adjacent tree is the same height or taller, calculate the viewing distance relative to the current tree and break the loop to move on to next offset
                if !(tree_grid[adjacent_row][adjacent_column] < tree_grid[row][column]) {
                    distances[i] = row.abs_diff(adjacent_row);
                    break;
                }

            // If only the column changes
            } else {
                match usize::checked_add_signed(adjacent_column, *column_offset) {
                    Some(some_column) => {
                        if some_column < width {
                            adjacent_column = some_column;
                        } else {
                            // Made it to the upper bound
                            distances[i] = width - 1 - column;
                            break;
                        }
                    }
                    None => {
                        // Made it to the lower bound
                        distances[i] = column;
                        break;
                    }
                }

                // if the adjacent tree is the same height or taller, calculate the viewing distance relative to the current tree and break the loop to move on to next offset
                if !(tree_grid[adjacent_row][adjacent_column] < tree_grid[row][column]) {
                    distances[i] = column.abs_diff(adjacent_column);
                    break;
                }
            }
        }
    }

    let mut scenic_score = 1;

    for distance in distances {
        scenic_score *= distance;
    }

    return scenic_score;
}

fn get_highest_scenic_score(tree_grid: &TreeGrid) -> usize {
    let mut highest_scenic_score = 0;

    for row in 0..tree_grid.len() {
        for column in 0..tree_grid[0].len() {
            let score = get_scenic_score(tree_grid, row, column);
            if score > highest_scenic_score {
                highest_scenic_score = score;
            }
        }
    }

    return highest_scenic_score;
}

fn part_2(tree_grid: &TreeGrid) {
    println!("Part 2: {}", get_highest_scenic_score(tree_grid));
}

fn main() {
    let file = File::open("./input.txt").expect("unable to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|r| r.unwrap()).collect();

    let height: usize = lines.len();
    let width: usize = lines[0].len();

    let mut tree_grid: TreeGrid = vec![vec![0_u8; width]; height];

    create_tree_grid(&lines, &mut tree_grid);

    part_1(&tree_grid);
    part_2(&tree_grid);
}
