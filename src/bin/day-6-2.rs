use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "inputs/input.6.txt";
    let contents = fs::read_to_string(file_path).expect(&format!("{} file not found", file_path));
    let lanternfish: Vec<u8> = contents.lines().next().unwrap().split(",").map(|s| s.parse().expect("NaN")).collect();

    let mut fish: HashMap<u8, u64> = HashMap::new();

    for f in lanternfish {
        *fish.entry(f).or_default() += 1;
    }

    for _i in 0..256 {
        let z = *fish.get(&0).unwrap_or(&0);

        for i in 1..=8 {
            *fish.entry(i-1).or_default() = *fish.get(&i).unwrap_or(&0);
        }

        *fish.entry(6).or_default() += z;
        *fish.entry(8).or_default() = z;
    }

    print!("Output: {}", fish.into_values().sum::<u64>());
}
