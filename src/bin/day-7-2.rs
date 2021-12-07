use std::collections::HashMap;
use std::fs;

fn fkt(index: i32, store: &mut HashMap<i32, i32>) -> i32 {
    if index <= 0 {
        return 0;
    }
    if index == 1 {
        return 1;
    }

    if let Some(x) = store.get(&index) {
        *x
    } else {
        let s = fkt(index - 1, store) + index;
        store.insert(index, s);
        return s;
    }
}

fn main() {
    let file_path = "inputs/input.7.txt";
    let contents = fs::read_to_string(file_path).expect(&format!("{} file not found", file_path));
    let crabs: Vec<i32> = contents.lines().next().unwrap().split(",").map(|s| s.parse().unwrap()).collect();

    let _min = *crabs.iter().min().unwrap();
    let _max = *crabs.iter().max().unwrap();

    // Height, fuel
    let mut heights: HashMap<i32, i32> = HashMap::new();

    let mut store = HashMap::new();

    for h in _min..=_max {
        let mut fuel = 0;

        for crab in &crabs {
            fuel += fkt((h - *crab).abs(), &mut store);
        }

        heights.insert(h, fuel);
    }

    println!("{:?}", heights);

    let smallest = *heights.values().min().unwrap();

    println!("Output: {}", smallest);
    // 97038163
}
