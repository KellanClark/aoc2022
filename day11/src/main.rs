
#[derive(Default)]
struct Monkey {
    items: std::collections::VecDeque<u64>,
    op_type: bool,
    op_param: u64,
    test_mod: u64,
    true_case: usize,
    false_case: usize,
}

fn solve(part2: bool) -> u64 {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut inspections: Vec<u64> = Vec::new();
    for _entry in std::fs::read_to_string("input").unwrap().split("\n\n") {
        let mut entry = _entry.lines().skip(1).peekable();
        monkeys.push(Monkey {
            items: entry.next().unwrap().strip_prefix("  Starting items: ").unwrap().split(", ").map(|x| x.parse::<u64>().unwrap()).collect(),
            op_type: entry.peek().unwrap().strip_prefix("  Operation: new = old ").unwrap().split_once(' ').unwrap().0 == "*",
            op_param: entry.next().unwrap().strip_prefix("  Operation: new = old ").unwrap().split_once(' ').unwrap().1.parse::<u64>().unwrap_or(0),
            test_mod: entry.next().unwrap().strip_prefix("  Test: divisible by ").unwrap().parse::<u64>().unwrap(),
            true_case: entry.next().unwrap().strip_prefix("    If true: throw to monkey ").unwrap().parse::<usize>().unwrap(),
            false_case: entry.next().unwrap().strip_prefix("    If false: throw to monkey ").unwrap().parse::<usize>().unwrap(),
        });
        inspections.push(0);
    }

    let mod_val: u64 = monkeys.iter().map(|x| x.test_mod).product();

    for _round in 0..match part2 { true => 10000, _ => 20 } {
         for monke in 0..monkeys.len() {
            while monkeys[monke].items.len() > 0 {
                inspections[monke] += 1;

                let mut worry = monkeys[monke].items.pop_front().unwrap();
                if monkeys[monke].op_type {
                    if monkeys[monke].op_param == 0 {
                        worry *= worry;
                    } else {
                        worry *= monkeys[monke].op_param;
                    }
                } else {
                    worry += monkeys[monke].op_param;
                }

                if part2 {
                    worry %= mod_val;
                } else {
                    worry /= 3;
                }

                if worry % monkeys[monke].test_mod == 0 {
                    let index = monkeys[monke].true_case;
                    monkeys[index].items.push_back(worry);
                } else {
                    let index = monkeys[monke].false_case;
                    monkeys[index].items.push_back(worry);
                }
            }
        }
    }

    inspections.sort_by(|a, b| b.cmp(a));
    inspections[0] * inspections[1]
}

fn main() {
    println!("Part 1: {}", solve(false));
    println!("Part 2: {}", solve(true));
}
