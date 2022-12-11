use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut visited1: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let mut visited9: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let mut segments: [(i32, i32); 10] = [(0, 0); 10];

    for line in input.lines() {
        let parsed = line.split_once(' ').unwrap();
        let distance = parsed.1.parse::<i32>().unwrap();
        let mut offset: (i32, i32) = match parsed.0 {
            "U" => (0, -distance),
            "D" => (0, distance),
            "L" => (-distance, 0),
            "R" => (distance, 0),
            _ => panic!()
        };

        while (offset.0 != 0) || (offset.1 != 0) {
            if offset.0 != 0 {
                segments[0].0 += offset.0.signum();
                offset.0 -= offset.0.signum();
            } else {
                segments[0].1 += offset.1.signum();
                offset.1 -= offset.1.signum();
            }

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
    }

    println!("Part 1: {}", visited1.len());
    println!("Part 2: {}", visited9.len());
}
