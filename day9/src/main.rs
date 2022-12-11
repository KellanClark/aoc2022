use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut visited1: HashSet<(i32, i32)> = HashSet::new();
    let mut visited9: HashSet<(i32, i32)> = HashSet::new();

    let mut segments = [(0, 0); 10];
    visited1.insert((0, 0));
    visited9.insert((0, 0));

    for line in input.lines() {
        let parsed = line.split_once(' ').unwrap();
        let distance = parsed.1.parse::<i32>().unwrap();
        match parsed.0 {
            "U" => segments[0].1 += distance,
            "D" => segments[0].1 -= distance,
            "L" => segments[0].0 -= distance,
            "R" => segments[0].0 += distance,
            _ => panic!()
        }
        //println!("{:?}", head_pos);
        for seg in 1..=9 {
            while ((segments[seg - 1].0 - segments[seg].0).abs() > 1) || ((segments[seg - 1].1 - segments[seg].1).abs() > 1) {
                if segments[seg - 1].0 != segments[seg].0 {
                    segments[seg].0 += (segments[seg - 1].0 - segments[seg].0).signum();
                }
                if segments[seg - 1].1 != segments[seg].1 {
                    segments[seg].1 += (segments[seg - 1].1 - segments[seg].1).signum();
                }

                if seg == 1 {
                    visited1.insert(segments[seg]);
                } else if seg == 9 {
                    visited9.insert(segments[seg]);
                }
            }
        }
    }

    println!("Part 1: {}", visited1.len());
    println!("Part 2: {}", visited9.len());
}
