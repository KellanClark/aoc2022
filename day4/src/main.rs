fn main() {
    let pairs: Vec<Vec<u32>> = std::fs::read_to_string("input").unwrap()
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(|c| c == ',' || c == '-').collect();
            let pair: Vec<u32> = split.into_iter().map(|x| x.parse::<u32>().unwrap()).collect();
            pair
        }).collect();

    let answer1 = pairs.iter().filter(|pair| (pair[0] <= pair[2] && pair[1] >= pair[3]) || (pair[2] <= pair[0] && pair[3] >= pair[1])).count();
    let answer2 = pairs.iter().filter(|pair| std::cmp::max(pair[0], pair[2]) <= std::cmp::min(pair[1], pair[3])).count();

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
