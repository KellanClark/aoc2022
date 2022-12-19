fn main() {
    let mut grid: Box<[[bool; 1000]; 600]> = Box::new([[false; 1000]; 600]);
    let mut void_level = 0;

    // Fill in lines
    for line in std::fs::read_to_string("input").unwrap().lines() {
        let mut coords = line.split(" -> ").map(|x| {
            let tmp = x.split_once(",").unwrap();
            (tmp.0.parse::<i32>().unwrap(), tmp.1.parse::<i32>().unwrap())
        });

        let mut pos = coords.next().unwrap();
        grid[pos.1 as usize][pos.0 as usize] = true;
        for next in coords {
            void_level = void_level.max(next.1 + 1);

            let offset = ((next.0 - pos.0).clamp(-1, 1), (next.1 - pos.1).clamp(-1, 1));
            while pos != next {
                pos.0 += offset.0;
                pos.1 += offset.1;

                grid[pos.1 as usize][pos.0 as usize] = true;
            }
        }
    }

    // Simulate sand
    let mut answer1 = 0;
    let mut answer2 = 0;
    loop {
        answer2 += 1;

        let mut pos = (500, 0);
        loop {
            if !grid[pos.1 + 1][pos.0] { // Down
                pos = (pos.0, pos.1 + 1);
            } else if !grid[pos.1 + 1][pos.0 - 1] { // Down left
                pos = (pos.0 - 1, pos.1 + 1);
            } else if !grid[pos.1 + 1][pos.0 + 1] { // Down right
                pos = (pos.0 + 1, pos.1 + 1);
            } else {
                break;
            }

            if (pos.1 == (void_level as usize)) || (pos == (500, 0)) {
                break;
            }
        }

        // Hit void
        if (answer1 == 0) && (pos.1 == (void_level as usize)) {
            answer1 = answer2 - 1;
        }
        // Hit source
        if pos == (500, 0) {
            break;
        } else {
            grid[pos.1][pos.0] = true;
        }
    }

    println!("Part 1: {answer1}");
    println!("Part 2: {answer2}");
}
