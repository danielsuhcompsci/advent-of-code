use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

// struct Node {
//     name: String,
//     size: usize,
//     children: Option<Box<Vec<Node>>>,
// }

// impl Node {
//     pub fn new(name: String, size: usize, children: Vec<Node>) -> Self {
//         Node {
//             name,
//             size,
//             children: Some(Box::new(children)),
//         }
//     }

//     pub fn add(mut self, child: Node) {
//         match self.children {
//             Some(mut children) => {
//                 children.push(child);
//             }
//             None => {
//                 let children: Vec<Node> = vec![child];
//                 self.children = Some(Box::new(children));
//             }
//         }
//     }
// }

// struct Tree {
//     root: Box<Node>,
// }

// impl Tree {
//     pub fn new(root: Node) -> Self {
//         Tree {
//             root: Box::new(root),
//         }
//     }
// }

fn part_1(lines: &Vec<String>) {
    // let root_children: Vec<Node> = Vec::new();
    // let tree = Tree::new(Node::new("root".to_string(), 0, root_children));

    let mut dir_to_size: HashMap<String, usize> = HashMap::new();
    let mut directory_stack: Vec<String> = Vec::new();

    let mut line_index = 1;

    while line_index < lines.len() {
        let split: Vec<&str> = lines[line_index].split(" ").collect();
        match split[0] {
            "$" => match split[1] {
                "cd" => {
                    directory_stack.push(split[2].to_string());
                }
                "ls" => {
                    continue;
                }
                _ => {
                    println!("Unable to read command '{}'", split[1]);
                }
            },
            "dir" => {
                dir_to_size.insert(split[1].to_string(), 0);
            }
            _ => {}
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
