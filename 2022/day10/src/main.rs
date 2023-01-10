use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

struct Process {
    pub value: i32,
    pub counter: usize,
}

impl Process {
    pub fn new(value: i32) -> Self {
        Process { value, counter: 2 }
    }

    pub fn decrement(&mut self) {
        self.counter -= 1;
    }

    pub fn is_done(&self) -> bool {
        self.counter == 0
    }
}

fn part_1(lines: &Vec<String>) {
    let mut sum = 0;
    let mut register: i32 = 1;

    let mut queue: VecDeque<Process> = VecDeque::new();

    for (i, line) in lines.iter().enumerate() {
        let split: Vec<&str> = line.split(' ').collect();
        match split[0] {
            "addx" => {
                let value: i32 = split[1].parse().unwrap();
                queue.push_back(Process::new(value));
            }
            _ => {}
        }

        // Calculate and add signal strength
        if i + 1 - 20 % 40 == 0 && i < 220 {
            sum += register * (i as i32 + 1);
        }

        match queue.front() {
            Some(process) => {
                if process.is_done() {
                    register += process.value;
                }

                queue.pop_front();
            }
            None => {}
        }

        for process in queue.iter_mut() {
            process.decrement();
        }
    }
}

fn part_2() {}

fn main() {
    let file = File::create("./input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|r| r.unwrap()).collect();
}
