use std::fs;

fn main() {
    let file_path = "inputs/input.1.txt";
    let contents = fs::read_to_string(file_path).expect(&format!("{} file not found", file_path));
    let numbers: Vec<i32> = contents.lines().map(|s| s.parse().expect("NaN")).collect();
    let mut count = 0;

    for i in 3..numbers.len() {
        let prev = numbers[i-3] + numbers[i-2] + numbers[i-1];
        let now = numbers[i-2] + numbers[i-1] + numbers[i];
        if now > prev {
            count += 1;
        }
    }

    print!("Output: {}", count);
}
