fn main() {
    let mut stacks9000: [Vec<char>; 9] = Default::default();
    let mut stacks9001: [Vec<char>; 9] = Default::default();

    let input = std::fs::read_to_string("input").unwrap();
    let input_split = input.split_once("\n\n").unwrap();

    for line in input_split.0.lines().rev().skip(1) {
        for c in line.char_indices().skip(1).step_by(4) {
            if c.1 != ' ' {
                stacks9000[c.0 / 4].push(c.1);
                stacks9001[c.0 / 4].push(c.1);
            }
        }
    }

    for line in input_split.1.lines() {
        let vals: Vec<usize> = line.split(' ').filter_map(|x| x.parse().ok()).collect();
        let len = vals[0];
        let start = vals[1] - 1;
        let end = vals[2] - 1;
        for _i in 0..len {
            let tmp = stacks9000[start].pop().unwrap();
            stacks9000[end].push(tmp);
        }
        let mut tmp = stacks9001[start].split_off(stacks9001[start].len() - len);
        stacks9001[end].append(&mut tmp);
    }

    print!("Part 1: ");
    stacks9000.map(|stack| print!("{}", stack.last().unwrap()));
    print!("\nPart 2: ");
    stacks9001.map(|stack| print!("{}", stack.last().unwrap()));
}
