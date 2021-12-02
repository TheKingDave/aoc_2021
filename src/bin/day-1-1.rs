use std::fs;

fn main() {
    let file_path = "inputs/input.1.txt";
    let contents = fs::read_to_string(file_path).expect(&format!("{} file not found", file_path));
    let numbers: Vec<i32> = contents.lines().map(|s| s.parse().expect("NaN")).collect();
    let mut count = 0;

    for i in 1..numbers.len() {
        if numbers[i] > numbers[i-1] {
            count += 1;
        }
    }

    print!("Output: {}", count);
}
