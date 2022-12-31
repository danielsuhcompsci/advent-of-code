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

fn sum_less_than(path_to_size: &HashMap<String, usize>) -> usize {
    let mut sum: usize = 0;

    for size in path_to_size.values() {
        if *size <= 100_000 {
            sum += size;
        }
    }

    return sum;
}

fn path_to_string(path: &[&str]) -> String {
    let mut path_string: String = String::new();

    if path.len() == 1 {
        path_string.push('/');
        return path_string;
    }

    for (i, string) in path.iter().enumerate() {
        if i == 1 {
            path_string.push_str(&string);
        } else {
            path_string.push('/');
            path_string.push_str(&string);
        }
    }
    return path_string;
}

fn part_1(lines: &Vec<String>) {
    let mut path: Vec<&str> = Vec::new();
    let mut path_to_size: HashMap<String, usize> = HashMap::new();

    for line in lines.iter() {
        println!("Line: {}", line);
        println!("Path: {:?}", path);
        let split: Vec<&str> = line.split(" ").collect();
        match split[0] {
            // A command
            "$" => match split[1] {
                "cd" => {
                    match split[2] {
                        "/" => {
                            // add root
                            path.push("/");
                            path_to_size.insert("/".to_string(), 0);
                        }
                        ".." => {
                            // move back in path
                            path.pop();
                        }
                        _ => {
                            println!("{}", line);
                            // add into stack
                            let name = split[2];
                            path.push(name);
                            path_to_size.insert(path_to_string(&path), 0);
                        }
                    }
                }
                "ls" => {
                    continue;
                }
                _ => {
                    // if there is an unknown command
                    println!("Unable to read command '{}'", split[1]);
                }
            },
            "dir" => {
                continue;
            }
            _ => {
                // add the file size to all directories in stack
                let size: usize = split[0].parse().unwrap();

                for i in (1..path.len() + 1).rev() {
                    // println!("I: {}", i);
                    // println!("{}", path_to_string(&path[..i]));
                    path_to_size.insert(
                        path_to_string(&path[..i]),
                        path_to_size[&path_to_string(&path[..i])] + size,
                    );
                }
            }
        }
    }

    println!("Part 1: {}", sum_less_than(&path_to_size));
}

fn part_2(lines: &Vec<String>) {}

fn main() {
    let file = File::open("input.txt").expect("unable to open input file");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().into_iter().map(|r| r.unwrap()).collect();

    part_1(&lines);
    part_2(&lines);
}
