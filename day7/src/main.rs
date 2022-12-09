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

fn fill(current_directory: &mut Directory, commands: &mut std::str::Split<'_, &str>) {
    loop {
        if let Some(x) = commands.next() {
            let command = x.split_once(&[' ', '\n'][..]).unwrap();
            if command.0 == "cd" {
                if command.1 == "..\n" {
                    return;
                } else {
                    fill(
                        current_directory
                            .files
                            .entry(command.1.trim().to_string())
                            .or_insert(Box::new(Directory::new())),
                        commands,
                    );
                }
            } else if command.0 == "ls" {
                for line in command.1.lines() {
                    let entry = line.split_once(' ').unwrap();
                    if entry.0 == "dir" {
                        current_directory
                            .files
                            .insert(entry.1.trim().to_string(), Box::new(Directory::new()));
                    } else {
                        current_directory.size += entry.0.parse::<u64>().unwrap();
                    }
                }
            }
        } else {
            return;
        }
    }
}

fn get_dir_size(
    current_directory: &mut Directory,
    answer1: &mut u64,
    answer2: &mut u64,
    space_to_free: u64,
) -> u64 {
    let mut total_size = current_directory.size;
    for (_name, dir) in &mut current_directory.files {
        total_size += get_dir_size(dir, answer1, answer2, space_to_free);
    }
    if total_size <= 100000 {
        *answer1 += total_size;
    }
    if (total_size >= space_to_free) && (total_size < *answer2) {
        *answer2 = total_size;
    }
    total_size
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut file_tree: Directory = Directory::new();
    let mut commands = input.split("$ ");
    commands.next();
    commands.next();
    fill(&mut file_tree, &mut commands);

    let mut answer1: u64 = 0;
    let mut answer2: u64 = 0;
    let used_space = get_dir_size(&mut file_tree, &mut answer1, &mut answer2, 0);
    answer1 = 0;
    answer2 = u64::MAX;
    get_dir_size(
        &mut file_tree,
        &mut answer1,
        &mut answer2,
        used_space - 40000000,
    );

    println!("Part 1: {answer1}");
    println!("Part 2: {answer2}");
}
