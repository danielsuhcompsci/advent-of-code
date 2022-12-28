use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader, Lines},
    vec::Vec,
};

fn parse(lines: &Vec<String>, stacks: &mut Vec<Vec<char>>) -> usize {
    let mut height: usize = 0;

    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();

        if split[0].parse::<u32>().is_ok() {
            for stack_index in 0..split.len() {
                let line_x_position: usize = stack_index * 4 + 1;
                for y in (0..height).rev() {
                    let character = lines[y].chars().nth(line_x_position).unwrap();
                    // println!("Y: {} Char: {}", y, character);
                    if character != ' ' {
                        match stacks.get_mut(stack_index) {
                            Some(vec) => vec.push(character),
                            None => {
                                let vec: Vec<char> = vec![character];
                                stacks.push(vec);
                            }
                        }
                    }
                }
            }

            return height;
        }

        height += 1;
    }

    return height;
}

fn get_tops(stacks: &Vec<Vec<char>>) -> String {
    let mut string: String = "".to_string();

    for stack in stacks.iter() {
        string.push(*stack.last().unwrap());
    }

    return string;
}

fn part_1(lines: &Vec<String>) {
    let mut stacks: Vec<Vec<char>> = Vec::new();

    let chart_end = parse(lines, &mut stacks);

    for i in (chart_end + 2)..lines.len() {
        let split: Vec<&str> = lines[i].split(' ').collect();
        let amount_to_move = split[1].parse::<usize>().unwrap();
        let from = split[3].parse::<usize>().unwrap() - 1;
        let to = split[5].parse::<usize>().unwrap() - 1;

        for _ in 0..amount_to_move {
            let popped = stacks[from].pop().unwrap();
            stacks[to].push(popped);
        }
    }

    println!("Part 1: {}", get_tops(&stacks));
}

fn part_2(lines: &Vec<String>) {
    let mut stacks: Vec<Vec<char>> = Vec::new();

    let chart_end = parse(lines, &mut stacks);
    for i in (chart_end + 2)..lines.len() {
        let split: Vec<&str> = lines[i].split(' ').collect();
        let amount_to_move = split[1].parse::<usize>().unwrap();
        let from = split[3].parse::<usize>().unwrap() - 1;
        let to = split[5].parse::<usize>().unwrap() - 1;

        let mut queue: VecDeque<char> = VecDeque::new();

        for _ in 0..amount_to_move {
            let popped = stacks[from].pop().unwrap();
            queue.push_front(popped);
        }

        for item in queue {
            stacks[to].push(item);
        }
    }

    println!("Part 2: {}", get_tops(&stacks));
}

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .into_iter()
        .map(|result| result.unwrap())
        .collect();

    part_1(&lines);
    part_2(&lines);
}
