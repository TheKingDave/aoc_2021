use std::fs;

fn main() {
    let file_path = "inputs/input.6.txt";
    let contents = fs::read_to_string(file_path).expect(&format!("{} file not found", file_path));
    let mut lanternfish: Vec<u8> = contents.lines().next().unwrap().split(",").map(|s| s.parse().expect("NaN")).collect();

    for _i in 0..80 {
        let mut to_add = 0;
        for fish in &mut lanternfish {
            if *fish == 0 {
                *fish = 6;
                to_add += 1;
            } else {
                *fish -= - 1;
            }
        }
        for _i in 0..to_add {
            lanternfish.push(8);
        }
    }

    print!("Output: {}", lanternfish.len());
}
