use std::collections::HashSet;

fn main() {
    let mut sensors: HashSet<(i32, i32)>;
    let mut beacons: HashSet<(i32, i32)>;

    for line in std::fs::read_to_string("input").unwrap().lines() {
        let vals: Vec<i32> = line.split(&['=', ',', ':']).filter_map(|x| x.parse::<i32>().ok()).collect();
        sensors.insert((vals[0], vals[1]));
        beacons.insert((vals[2], vals[3]));
    }
}
