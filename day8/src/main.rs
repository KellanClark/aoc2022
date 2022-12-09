use std::cmp::max;

fn main() {
    let mut heights: Vec<Vec<i32>> = Vec::new();
    for line in std::fs::read_to_string("input").unwrap().lines() {
        heights.push(line.chars().map(|c| (c as i32) - ('0' as i32)).collect());
    }
    let r_size = heights.len();
    let c_size = heights[0].len();

    let mut visible: Vec<Vec<bool>> = vec![vec![false; c_size]; r_size];
    for row in 0..r_size {
        let mut max = -1;
        for column in 0..c_size {
            let height = heights[row][column];
            if height > max {
                visible[row][column] = true;
                max = height;
            }
        }

        max = -1;
        for column in 0..c_size {
            let height = heights[row][c_size - 1 - column];
            if height > max {
                visible[row][c_size - 1 - column] = true;
                max = height;
            }
        }
    }
    for column in 0..c_size {
        let mut max = -1;
        for row in 0..r_size {
            let height = heights[row][column];
            if height > max {
                visible[row][column] = true;
                max = height;
            }
        }

        max = -1;
        for row in 0..r_size {
            let height = heights[r_size - 1 - row][column];
            if height > max {
                visible[r_size - 1 - row][column] = true;
                max = height;
            }
        }
    }

    let mut answer1 = 0;
    let mut answer2 = 0;
    for row in 0..r_size {
        for column in 0..c_size {
            if visible[row][column] {
                answer1 += 1;
            }

            if (column > 0) && (row > 0) && (column < c_size - 1) && (row < r_size - 1) {
                let height = heights[row][column];
                let mut up_score: usize = 1;
                while height > heights[row - up_score][column] { if (row - up_score) != 0 { up_score += 1; } else { break; } }
                let mut down_score: usize = 1;
                while height > heights[row + down_score][column] { if (row + down_score) != (r_size - 1) { down_score += 1; } else { break; } }
                let mut left_score: usize = 1;
                while height > heights[row][column - left_score] { if (column - left_score) != 0 { left_score += 1; } else { break; } }
                let mut right_score: usize = 1;
                while height > heights[row][column + right_score] { if (column + right_score) != (c_size - 1) { right_score += 1; } else { break; } }
                let score = max(up_score, 1) * max(down_score, 1) * max(left_score, 1) * max(right_score, 1);
                if score > answer2 {
                    answer2 = score;
                }
            }
        }
    }

    println!("Part 1: {answer1}");
    println!("Part 2: {answer2}");
}
