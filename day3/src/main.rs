fn main() {
    let answer1 = std::fs::read_to_string("input").unwrap().lines().map(|line| {
        let p: char = line.chars().nth(line[(line.len() / 2)..].chars().map(|c| line[..(line.len() / 2)].find(c).unwrap_or(0)).max().unwrap()).unwrap();
        match p {
            'a'..='z' => (p as u64) - 96,
            'A'..='Z' => (p as u64) - 64 + 26,
            _ => panic!()
        }
    }).sum::<u64>();

    let binding = std::fs::read_to_string("input").unwrap();
    let lines: Vec<&str> = binding.lines().collect();
    let mut i = 0;
    let mut answer2 = 0;
    while i < lines.len() {
        let mut p: char = ' ';
        for c in lines[i].chars() {
            if lines[i + 1].contains(c) && lines[i + 2].contains(c) {
                p = c;
            }
        }
        assert!(p != ' ');

        answer2 += match p {
            'a'..='z' => (p as u64) - 96,
            'A'..='Z' => (p as u64) - 64 + 26,
            _ => panic!()
        };

        i += 3;
    }
    //let test: Vec<&str> = std::fs::read_to_string("input").unwrap().lines().collect();
    /*let answer2 = lines.as_slice().chunks_exact(3).map(|group| {
        let match21 = group[2].chars().map(|c| group[1].find(c).unwrap_or(696969)).filter(|x| x != 696969);
        let p: char = group[0].chars().nth(match21.map(|c| group[0].find(c).unwrap_or(0)).max().unwrap()).unwrap();
        match p {
            'a'..='z' => (p as u64) - 96,
            'A'..='Z' => (p as u64) - 64 + 26,
            _ => panic!()
        }
    }).sum::<u64>();*/

    println!("Part 1: {}\n", answer1);
    println!("Part 2: {}\n", answer2);
}
