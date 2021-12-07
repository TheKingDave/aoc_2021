use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "inputs/input.7.txt";
    let contents = fs::read_to_string(file_path).expect(&format!("{} file not found", file_path));
    let crabs: Vec<i32> = contents.lines().next().unwrap().split(",").map(|s| s.parse().unwrap()).collect();

    let _min = *crabs.iter().min().unwrap();
    let _max = *crabs.iter().max().unwrap();

    // Height, fuel
    let mut heights: HashMap<i32, i32> = HashMap::new();

    for h in _min..=_max {
        let mut fuel = 0;

        for crab in &crabs {
            fuel += (h - *crab).abs();
        }

        heights.insert(h, fuel);
    }

    let smallest = *heights.values().min().unwrap();

    println!("Output: {}", smallest);
    // 345035
}
