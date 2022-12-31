use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

// #[derive(Debug, Clone)]
// struct Node {
//     name: String,
//     size: usize,
//     children: Option<Vec<Box<Node>>>,
// }

// impl Node {
//     pub fn new(name: String, size: usize, children: Option<Vec<Box<Node>>>) -> Self {
//         match children {
//             Some(children) => Node {
//                 name,
//                 size,
//                 children: Some(children),
//             },
//             None => Node {
//                 name,
//                 size,
//                 children: None,
//             },
//         }
//     }

//     pub fn add(mut self, child: Box<Node>) {
//         match self.children {
//             Some(mut children) => {
//                 children.push(child);
//             }
//             None => {
//                 let children: Vec<Box<Node>> = vec![child];
//                 self.children = Some(children);
//             }
//         }
//     }

//     pub fn next(self, name: String) -> Option<Box<Node>> {
//         for child in self.children.unwrap() {
//             if child.name == name {
//                 return Some(child);
//             }
//         }

//         return None;
//     }
// }

// #[derive(Debug, Clone)]
// struct Tree {
//     pub root: Box<Node>,
//     pub sum: usize,
// }

// impl Tree {
//     pub fn new(root: Node) -> Self {
//         Tree {
//             root: Box::new(root),
//             sum: 0,
//         }
//     }

//     pub fn sum_size(&mut self, current: Box<Node>) -> usize {
//         match current.children {
//             Some(children) => {
//                 let mut size: usize = 0;
//                 for child in children {
//                     size += Self::sum_size(self, child);
//                 }
//                 if size <= 100000 {
//                     self.sum += size;
//                 }
//                 size
//             }
//             None => current.size,
//         }
//     }
// }

fn build_tree(lines: &Vec<String>, tree: &Tree) {
    let mut line_index = 1;

    let mut current = tree.root.clone();

    while line_index < lines.len() {
        println!("Line: {}", line_index + 1);
        let split: Vec<&str> = lines[line_index].split(" ").collect();
        match split[0] {
            // A command
            "$" => match split[1] {
                "cd" => {
                    let name = split[2];
                    println!("Command 'cd'; Current: {:?}", current);
                    match current.clone().next(name.to_string()) {
                        Some(next) => {
                            current = next;
                        }
                        None => {
                            println!("Unable to enter directory {}.", name);
                        }
                    }
                }
                "ls" => {}
                _ => {
                    // if there is an unknown command
                    println!("Unable to read command '{}'", split[1]);
                }
            },
            "dir" => {
                // add the directory to current directory
                let name = split[1].to_string();
                current.clone().add(Box::new(Node::new(name, 0, None)));
            }
            _ => {
                // add the file to the current directory
                let size: usize = split[0].parse().unwrap();
                let name = split[1].to_string();

                current.clone().add(Box::new(Node::new(name, size, None)));
            }
        }

        line_index += 1;
    }
}

fn part_1(lines: &Vec<String>) {
    let mut tree = Tree::new(Node::new("root".to_string(), 0, None));

    let current = &mut tree.root;

    let test_node = Box::new(Node::new("test".to_string(), 0, None));

    current.add(test_node);

    // current.add(Box::new(Node::new("test".to_string(), 0, None)));
    // current.add(Box::new(Node::new("test".to_string(), 0, None)));

    println!("Tree: {:?}", tree);
    // println!("Current: {:?}", current.children);

    build_tree(lines, &tree);

    println!("Tree Built");
    println!("Part 1: {}", tree.sum_size(tree.root.clone()));
}

fn part_2(lines: &Vec<String>) {}

fn main() {
    let file = File::open("input.txt").expect("unable to open input file");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().into_iter().map(|r| r.unwrap()).collect();

    part_1(&lines);
    part_2(&lines);
}
