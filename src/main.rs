use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("intput.txt file not found");
    let numbers: Vec<i32> = contents.lines().map(|s| s.parse().expect("NaN")).collect();
    let mut count = 0;

    for i in 1..numbers.len() {
        if numbers[i-1] < numbers[i] {
            count += 1;
        }
    }

    print!("Output: {}", count);
}
