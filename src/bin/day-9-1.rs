use std::fs;

fn main() {
    let file_path = "inputs/input.9.txt";
    let contents = fs::read_to_string(file_path).expect(&format!("{} file not found", file_path));

    let grid: Vec<Vec<u8>> = contents.lines().map(|s| s.chars().map(|x| x.to_digit(10).unwrap() as u8).collect()).collect();

    let i_len = grid.len();
    let a_len = grid.first().unwrap().len();

    let mut sum: u32 = 0;

    for (i, line) in grid.iter().enumerate() {
        for (a, element) in line.iter().enumerate() {
            let mut is_low = true;

            if a != 0 {
                let left = line.get(a-1).unwrap();
                if left <= element {
                    is_low = false;
                }
            }
            if a+1 < a_len {
                let right = line.get(a+1).unwrap();
                if right <= element {
                    is_low = false;
                }
            }
            if i != 0 {
                let up = grid.get(i-1).unwrap().get(a).unwrap();
                if up <= element {
                    is_low = false;
                }
            }
            if i+1 < i_len {
                let down = grid.get(i+1).unwrap().get(a).unwrap();
                if down <= element {
                    is_low = false;
                }
            }
            if is_low {
                sum += 1 + (*element as u32);
            }

        }
    }

    print!("Output: {}", sum);
}
