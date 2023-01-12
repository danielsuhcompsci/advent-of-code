use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

// struct Process {
//     pub value: i32,
//     pub counter: usize,
// }

// impl Process {
//     pub fn new(value: i32) -> Self {
//         Process { value, counter: 2 }
//     }

//     pub fn decrement(&mut self) {
//         self.counter -= 1;
//     }

//     pub fn is_done(&self) -> bool {
//         self.counter == 0
//     }
// }

fn calc_signal_strength_if(sum: &mut i32, register: &i32, cycle: &usize) {
    // Calculate and add signal strength if 20th, 60th, 100th ... 220th cycle
    if (*cycle as i32 - 20) % 40 == 0 && *cycle <= 220 {
        *sum += register * (*cycle as i32);
    }
}

fn part_1(lines: &Vec<String>) {
    let mut sum = 0;
    let mut register: i32 = 1;

    // let mut queue: VecDeque<Process> = VecDeque::new();

    let mut cycle: usize = 1;

    for line in lines.iter() {
        calc_signal_strength_if(&mut sum, &register, &cycle);

        // Parse input
        let split: Vec<&str> = line.split(' ').collect();
        match split[0] {
            "addx" => {
                let value: i32 = split[1].parse().unwrap();

                cycle += 1;
                calc_signal_strength_if(&mut sum, &register, &cycle);

                cycle += 1;
                register += value;
            }
            _ => {
                cycle += 1;
            }
        }
    }

    println!("Part 1: {}", sum);
}

fn print_pixel(sprite_position: i32, cycle: usize) {
    let cycle_x = match cycle % 40 {
        0 => cycle - 1,
        _ => cycle % 40 - 1,
    };

    if sprite_position - 1 <= cycle_x as i32 && cycle_x as i32 <= sprite_position + 1 {
        print!("#");
    } else {
        print!(".");
    }

    if cycle % 40 == 0 {
        println!();
    }
}

fn part_2(lines: &Vec<String>) {
    println!("Part 2:");

    let mut register: i32 = 1;

    // let mut queue: VecDeque<Process> = VecDeque::new();

    let mut cycle: usize = 1;

    for line in lines.iter() {
        print_pixel(register, cycle);

        // Parse input
        let split: Vec<&str> = line.split(' ').collect();
        match split[0] {
            "addx" => {
                let value: i32 = split[1].parse().unwrap();

                cycle += 1;

                print_pixel(register, cycle);

                cycle += 1;
                register += value;
            }
            _ => {
                cycle += 1;
            }
        }
    }
}

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|r| r.unwrap()).collect();

    part_1(&lines);
    part_2(&lines);
}
