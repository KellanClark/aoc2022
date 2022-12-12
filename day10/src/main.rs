fn main() {
    let mut x = 1;
    let mut cycle: i32 = 0;
    let mut score = 0;
    let mut screen = [[false; 40]; 6];

    for line in std::fs::read_to_string("input").unwrap().lines() {
        cycle += 1;
        if (cycle % 40) == 20 {
            score += x * cycle;
        }
        if (((cycle - 1) % 40) - x).abs() <= 1 {
            screen[(cycle - 1) as usize / 40][(cycle - 1) as usize % 40] = true;
        }

        if line != "noop" {
            cycle += 1;
            if (cycle % 40) == 20 {
                score += x * cycle;
            }
            if (((cycle - 1) % 40) - x).abs() <= 1 {
                screen[(cycle - 1) as usize / 40][(cycle - 1) as usize % 40] = true;
            }

            x += line.split_once(' ').unwrap().1.parse::<i32>().unwrap();
        }
    }

    println!("Part 1: {}", score);
    println!("Part 2:");
    for y in 0..6 {
        for x in 0..40 {
            if screen[y][x] {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}
