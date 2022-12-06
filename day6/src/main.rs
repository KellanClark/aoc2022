fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut answer1 = 0;
    let mut answer2 = 0;
    let mut buf = ['\0'; 14];
    for c in input.chars() {
        answer2 += 1;
        for i in 0..=12 {
            buf[13 - i] = buf[13 - i - 1];
        }
        buf[0] = c;

        if (answer1 == 0) && (answer2 >= 4) && (buf[0] != buf[1]) && (buf[0] != buf[2]) && (buf[0] != buf[3]) && (buf[1] != buf[2]) && (buf[1] != buf[3]) && (buf[2] != buf[3]) {
            answer1 = answer2;
        }
        if (answer2 >= 14) &&
            (buf[ 0] != buf[13]) && (buf[ 0] != buf[12]) && (buf[ 0] != buf[11]) && (buf[ 0] != buf[10]) && (buf[ 0] != buf[9]) && (buf[ 0] != buf[8]) && (buf[ 0] != buf[7]) && (buf[ 0] != buf[6]) && (buf[ 0] != buf[5]) && (buf[ 0] != buf[4]) && (buf[ 0] != buf[3]) && (buf[ 0] != buf[2]) && (buf[ 0] != buf[1]) &&
            (buf[ 1] != buf[13]) && (buf[ 1] != buf[12]) && (buf[ 1] != buf[11]) && (buf[ 1] != buf[10]) && (buf[ 1] != buf[9]) && (buf[ 1] != buf[8]) && (buf[ 1] != buf[7]) && (buf[ 1] != buf[6]) && (buf[ 1] != buf[5]) && (buf[ 1] != buf[4]) && (buf[ 1] != buf[3]) && (buf[ 1] != buf[2]) &&
            (buf[ 2] != buf[13]) && (buf[ 2] != buf[12]) && (buf[ 2] != buf[11]) && (buf[ 2] != buf[10]) && (buf[ 2] != buf[9]) && (buf[ 2] != buf[8]) && (buf[ 2] != buf[7]) && (buf[ 2] != buf[6]) && (buf[ 2] != buf[5]) && (buf[ 2] != buf[4]) && (buf[ 2] != buf[3]) &&
            (buf[ 3] != buf[13]) && (buf[ 3] != buf[12]) && (buf[ 3] != buf[11]) && (buf[ 3] != buf[10]) && (buf[ 3] != buf[9]) && (buf[ 3] != buf[8]) && (buf[ 3] != buf[7]) && (buf[ 3] != buf[6]) && (buf[ 3] != buf[5]) && (buf[ 3] != buf[4]) &&
            (buf[ 4] != buf[13]) && (buf[ 4] != buf[12]) && (buf[ 4] != buf[11]) && (buf[ 4] != buf[10]) && (buf[ 4] != buf[9]) && (buf[ 4] != buf[8]) && (buf[ 4] != buf[7]) && (buf[ 4] != buf[6]) && (buf[ 4] != buf[5]) &&
            (buf[ 5] != buf[13]) && (buf[ 5] != buf[12]) && (buf[ 5] != buf[11]) && (buf[ 5] != buf[10]) && (buf[ 5] != buf[9]) && (buf[ 5] != buf[8]) && (buf[ 5] != buf[7]) && (buf[ 5] != buf[6]) &&
            (buf[ 6] != buf[13]) && (buf[ 6] != buf[12]) && (buf[ 6] != buf[11]) && (buf[ 6] != buf[10]) && (buf[ 6] != buf[9]) && (buf[ 6] != buf[8]) && (buf[ 6] != buf[7]) &&
            (buf[ 7] != buf[13]) && (buf[ 7] != buf[12]) && (buf[ 7] != buf[11]) && (buf[ 7] != buf[10]) && (buf[ 7] != buf[9]) && (buf[ 7] != buf[8]) &&
            (buf[ 8] != buf[13]) && (buf[ 8] != buf[12]) && (buf[ 8] != buf[11]) && (buf[ 8] != buf[10]) && (buf[ 8] != buf[9]) &&
            (buf[ 9] != buf[13]) && (buf[ 9] != buf[12]) && (buf[ 9] != buf[11]) && (buf[ 9] != buf[10]) &&
            (buf[10] != buf[13]) && (buf[10] != buf[12]) && (buf[10] != buf[11]) &&
            (buf[11] != buf[13]) && (buf[11] != buf[12]) &&
            (buf[12] != buf[13]) {
            break;
        }
    };

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
