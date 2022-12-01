use std::fs;

fn main() {
    //let answer1: u64 = std::fs::read_to_string("input").unwrap().split("\n\n").map(|elf| elf.lines().map(|x| x.parse::<u64>().unwrap()).sum()).max().unwrap();

    let mut elfs: Vec<u64> = fs::read_to_string("input").unwrap().split("\n\n").map(|elf| elf.lines().map(|x| x.parse::<u64>().unwrap()).sum()).collect();
    elfs.sort();
    elfs.reverse();
    let answer1: u64 = elfs[0];
    let answer2: u64 = elfs[..3].into_iter().sum();

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
