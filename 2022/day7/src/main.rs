use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

struct Node {
    name: String,
    size: usize,
    children: Option<Box<Vec<Node>>>,
}

impl Node {
    pub fn new(name: String, size: usize, children: Vec<Node>) -> Self {
        Node {
            name,
            size,
            children: Some(Box::new(children)),
        }
    }

    pub fn add(mut self, child: Node) {
        match self.children {
            Some(mut children) => {
                children.push(child);
            }
            None => {
                let children: Vec<Node> = vec![child];
                self.children = Some(Box::new(children));
            }
        }
    }
}

struct Tree {
    root: Box<Node>,
}

impl Tree {
    pub fn new(root: Node) -> Self {
        Tree {
            root: Box::new(root),
        }
    }

    pub fn sum_less_than(bound: usize) {}
}

fn part_1(lines: &Vec<String>) {
    // let root_children: Vec<Node> = Vec::new();
    // let tree = Tree::new(Node::new("root".to_string(), 0, root_children));

    // Holds the map of directory to size
    let mut directory_to_size: HashMap<String, usize> = HashMap::new();

    // The current directory at the top of the stack
    let mut directory_stack: Vec<String> = Vec::new();

    let mut line_index = 1;

    while line_index < lines.len() {
        let split: Vec<&str> = lines[line_index].split(" ").collect();
        match split[0] {
            "$" => match split[1] {
                // A command
                "cd" => {
                    // push it onto the stack to make it the current directory
                    directory_stack.push(split[2].to_string());
                }
                "ls" => {
                    // no action required
                    continue;
                }
                _ => {
                    // if there is an unknown command
                    println!("Unable to read command '{}'", split[1]);
                }
            },
            "dir" => {
                // add the directory to the map
                directory_to_size.insert(split[1].to_string(), 0);
            }
            _ => {
                // add the size of the file to the current directory
                let size: usize = split[0].parse().unwrap();
                let current_directory = directory_stack.last().unwrap();
                directory_to_size.insert(
                    (*current_directory).clone(),
                    directory_to_size[current_directory] + size,
                );
            }
        }
    }
}

fn part_2(lines: &Vec<String>) {}

fn main() {
    let file = File::open("input.txt").expect("unable to open input file");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().into_iter().map(|r| r.unwrap()).collect();

    part_1(&lines);
    part_2(&lines);
}
