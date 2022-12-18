fn compare_chars(left: char, right: char) -> Option<bool> {
    assert!(left != '[' && left != ']' && left != ',');
    assert!(right != '[' && right != ']' && right != ',');
    if (left as u8) < (right as u8) {
        Some(true)
    } else if (left as u8) > (right as u8) {
        Some(false)
    } else {
        None
    }
}

fn check_list(left: &mut Vec<char>, left_index: &mut usize, right: &mut Vec<char>, right_index: &mut usize) -> Option<bool> {
    loop {
        let mut left_char = left[*left_index]; *left_index += 1;
        let mut right_char = right[*right_index]; *right_index += 1;

        // Workaound for the number 10
        if (left_char == '1') && (left[*left_index] == '0'){
            *left_index -= 1;
            left.remove(*left_index);
            left[*left_index] = ':';
            left_char = ':';
        }
        if (right_char == '1') && (right[*right_index] == '0'){
            *right_index -= 1;
            right.remove(*right_index);
            right[*right_index] = ':';
            right_char = ':';
        }

        // Skip commas
        if left_char == ',' {
            left_char = left[*left_index]; *left_index += 1;
        }
        if right_char == ',' {
            right_char = right[*right_index]; *right_index += 1;
        }

        if left_char == ']' && right_char == ']' { // Lists end at same time
            return None;
        } else if left_char == ']' && right_char != ']' { // Left ends before right
            return Some(true);
        } else if left_char != ']' && right_char == ']' { // Right ends before left
            return Some(false);
        } else if left_char == '[' && right_char == '[' { // Both are lists
            let result = check_list(left, left_index, right, right_index);
            if result.is_some() {
                return result;
            }
        } else if left_char == '[' && right_char != '[' { // Left is list; right is int
            right.insert(*right_index + 1, ']');
            right.insert(*right_index - 1, '[');
            *left_index -= 1;
            *right_index -= 1;
        } else if left_char != '[' && right_char == '[' { // Right is list; left is int
            left.insert(*left_index + 1, ']');
            left.insert(*left_index - 1, '[');
            *left_index -= 1;
            *right_index -= 1;
        } else { // Both are ints
            let result = compare_chars(left_char, right_char);
            if result.is_some() {
                return result;
            }
        }
    }
}

fn main() {
    let mut packets: Vec<Vec<char>> = Vec::new();
    for text in std::fs::read_to_string("input").unwrap().split("\n\n") {
        let pair = text.split_once('\n').unwrap();
        packets.push(pair.0.chars().collect());
        packets.push(pair.1.chars().collect());
    }

    let mut answer1 = 0;
    for index in 0..(packets.len() / 2) {
        let mut left_index: usize = 1;
        let mut right_index: usize = 1;
        let mut left: Vec<char> = packets[index * 2].clone();
        let mut right: Vec<char> = packets[(index * 2) + 1].clone();

        if check_list(&mut left, &mut left_index, &mut right, &mut right_index).unwrap() {
            answer1 += index + 1;
        }
    }

    // Add dividers
    packets.push("[[2]]".chars().collect());
    packets.push("[[6]]".chars().collect());

    // Sort packets
    for i in 0..packets.len() {
        for j in 0..packets.len() - i - 1 {
            let mut left_index: usize = 1;
            let mut right_index: usize = 1;
            let mut left: Vec<char> = packets[j].clone();
            let mut right: Vec<char> = packets[j + 1].clone();

            if !check_list(&mut left, &mut left_index, &mut right, &mut right_index).unwrap() {
                packets[j] = right;
                packets[j + 1] = left;
            }
        }
    }

    // Find dividers again
    let mut divider2: Vec<char> = "[[2]]".chars().collect();
    let mut divider6: Vec<char> = "[[6]]".chars().collect();
    let mut answer2 = 1;
    for i in 0..packets.len() {
        let mut left_index: usize = 1;
        let mut right_index: usize = 1;
        if check_list(&mut divider2, &mut left_index, &mut packets[i], &mut right_index).is_none() {
            answer2 *= i + 1;
        }

        left_index = 1;
        right_index = 1;
        if check_list(&mut divider6, &mut left_index, &mut packets[i], &mut right_index).is_none() {
            answer2 *= i + 1;
        }
    }

    println!("Part 1: {answer1}");
    println!("Part 2: {answer2}");
}
