use std::fs;

fn count(list: &Vec<Vec<bool>>, index: usize) -> (u32, u32) {
    let len = list.len() as u32;
    let mut ones = 0;
    for num in list {
        if num[index] == true { ones += 1; }
    }
    let zeros = len - ones;
    (ones, zeros)
}

fn main() {
    let file_path = "inputs/input.3.txt";
    let contents = fs::read_to_string(file_path).expect(&format!("{} file not found", file_path));
    let lines: Vec<Vec<bool>> = contents.lines().map(|s| s.chars().map(|c| c == '1').collect()).collect();

    let line_len = lines.first().unwrap().len();

    let mut ox_lines = lines.clone();
    let mut index = 0;
    while ox_lines.len() > 1 {
        let (ones, zeros) = count(&ox_lines, index);
        let most = if ones == zeros { true } else { ones > zeros };
        ox_lines = ox_lines.into_iter().filter(|num| num[index] == most).collect();
        index += 1;
    }

    let mut sc_lines = lines.clone();
    let mut index = 0;
    while sc_lines.len() > 1 {
        let (ones, zeros) = count(&sc_lines, index);
        let most = if ones == zeros { false } else { !(ones > zeros) };
        sc_lines = sc_lines.into_iter().filter(|num| num[index] == most).collect();
        index += 1;
    }

    let oxygen_num = ox_lines.first().unwrap();
    let scrubber_num = sc_lines.first().unwrap();

    let mut oxygen: u32 = 0;
    let mut scrubber: u32 = 0;

    for i in 0..line_len {
        oxygen |= (oxygen_num[i] as u32) << (line_len - i - 1);
        scrubber |= (scrubber_num[i] as u32) << (line_len - i - 1);
    }

    println!("Output: {}", oxygen * scrubber);
}
