use std::fs;

fn main() {
    let file_path = "inputs/input.3.txt";
    let contents = fs::read_to_string(file_path).expect(&format!("{} file not found", file_path));
    let lines: Vec<&str> = contents.lines().collect();

    let line_len = lines.first().unwrap().len();
    let total_lines = lines.len();
    let mut count = vec![0; line_len];

    for line in lines {
        for (i, char) in line.chars().into_iter().enumerate() {
            if char == '1' {
                count[i] += 1;
            }
        }
    }

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    let line_len_half = total_lines / 2;
    for i in 0..line_len {
        if count[i] > line_len_half {
            gamma |= 1 << (line_len - i - 1);
        } else {
            epsilon |= 1 << (line_len - i - 1);
        }
    }

    println!("Output: {}", gamma * epsilon);
}
