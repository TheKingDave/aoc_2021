use std::fs;

fn main() {
    let file_path = "inputs/input.8.txt";
    let contents = fs::read_to_string(file_path).expect(&format!("{} file not found", file_path));

    let mut _digits: Vec<(Vec<&str>, Vec<&str>)> = Vec::new();
    let mut count = 0;

    for line in contents.trim().lines() {
        let mut parts = line.split("|").map(|x| x.trim());

        parts.next();

        let s = parts.next().unwrap().split_whitespace();
        for p in s {
            let len = p.len();
            if len == 2 || len == 3 || len == 4 || len == 7 {
                count += 1;
            }
        }
    }

    print!("Output: {}", count);
}
