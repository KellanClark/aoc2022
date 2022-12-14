
struct Node {
    height: i32,
    distance: i32,
}

fn explore(grid: &mut Vec<Vec<Node>>, current_pos: (usize, usize), current_distance: i32) {
    grid[current_pos.1][current_pos.0].distance = current_distance;
    let height = grid[current_pos.1][current_pos.0].height;

    if (current_pos.0 > 0) && (grid[current_pos.1][current_pos.0 - 1].height <= (height + 1)) && (grid[current_pos.1][current_pos.0 - 1].distance > (current_distance + 1)) {
        explore(grid, (current_pos.0 - 1, current_pos.1), current_distance + 1);
    }
    if (current_pos.0 < grid[current_pos.1].len() - 1) && (grid[current_pos.1][current_pos.0 + 1].height <= (height + 1)) && (grid[current_pos.1][current_pos.0 + 1].distance > (current_distance + 1)) {
        explore(grid, (current_pos.0 + 1, current_pos.1), current_distance + 1);
    }
    if (current_pos.1 > 0) && (grid[current_pos.1 - 1][current_pos.0].height <= (height + 1)) && (grid[current_pos.1 - 1][current_pos.0].distance > (current_distance + 1)) {
        explore(grid, (current_pos.0, current_pos.1 - 1), current_distance + 1);
    }
    if (current_pos.1 < grid.len() - 1) && (grid[current_pos.1 + 1][current_pos.0].height <= (height + 1)) && (grid[current_pos.1 + 1][current_pos.0].distance > (current_distance + 1)) {
        explore(grid, (current_pos.0, current_pos.1 + 1), current_distance + 1);
    }
}

fn main() {
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    let mut grid: Vec<Vec<Node>> = Vec::new();

    for line in std::fs::read_to_string("input").unwrap().lines() {
        let mut row: Vec<Node> = Vec::new();
        for c in line.chars() {
            let val = match c {
                'S' => {
                    start_pos = (row.len(), grid.len());
                    0
                },
                'E' => {
                    end_pos = (row.len(), grid.len());
                    25
                },
                _ => (c as i32) - ('a' as i32)
            };
            row.push(Node {
                height: val,
                distance: i32::MAX,
            });
        }
        grid.push(row);
    }

    explore(&mut grid, start_pos, 0);
    println!("Part 1: {}", grid[end_pos.1][end_pos.0].distance);

    let mut answer2 = i32::MAX;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x].height == 0 {
                explore(&mut grid, (x, y), 0);
                answer2 = answer2.min(grid[end_pos.1][end_pos.0].distance);
            }
        }
    }
    println!("Part 2: {}", answer2);
}
