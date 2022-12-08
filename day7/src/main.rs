use std::collections::HashMap;

struct Directory {
    size: u64,
    files: HashMap<String, Box<Directory>>,
}

impl Directory {
    fn new() -> Self {
        Self {
            size: 0,
            files: HashMap::new(),
        }
    }
}

fn fill(current_directory: &mut Directory, mut commands: std::str::Split<'_, &str>) {
    let command = match commands.next() {
        None => return,
        Some(x) => {println!("{x}"); x.split_once(' ').unwrap()},
    };

    if command.0 == "cd" {
        if command.1 == ".." {
            return;
        } else {
            fill(current_directory.files.entry(command.1.to_string()).or_insert(Box::new(Directory::new())), commands);
        }
    } else if command.0 == "ls" {
        for line in command.1.lines().skip(1) {
            let entry = line.split_once(' ').unwrap();
            if entry.0 == "dir" {
                current_directory.files.insert(entry.0.to_string(), Box::new(Directory::new()));
            } else {
                current_directory.size += entry.0.parse::<u64>().unwrap();
            }
        }
    } else {
        return;
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("Hello, world! {:?}", input.split(' '));

    let mut file_tree: Directory = Directory::new();
    fill(&mut file_tree, input.split("$ "));

    println!("Hello, world! {input}");
}
