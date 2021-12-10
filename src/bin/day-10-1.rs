use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "inputs/input.10.txt";
    let contents = fs::read_to_string(file_path).expect(&format!("{} file not found", file_path));
    let lines: Vec<_> = contents.lines().collect();

    let mut illegal_chars: HashMap<char, u32> = HashMap::new();

    let mut open_close: HashMap<char,char> = HashMap::new();
    open_close.insert('(', ')');
    open_close.insert('[', ']');
    open_close.insert('{', '}');
    open_close.insert('<', '>');

    for line in lines {
        let mut char_stack: Vec<char> = Vec::new();

        for c in line.chars() {
            if open_close.contains_key(&c) {
                char_stack.push(c);
            } else {
                if let Some(last) = char_stack.pop() {
                    if c != *open_close.get(&last).unwrap() {
                        *illegal_chars.entry(c).or_default() += 1;
                    }
                }
            }
        }
    }

    println!("{:?}", illegal_chars);

    let mut point_map: HashMap<char,u32> = HashMap::new();
    point_map.insert(')', 3);
    point_map.insert(']', 57);
    point_map.insert('}', 1197);
    point_map.insert('>', 25137);

    let mut sum = 0;
    for e in illegal_chars {
        sum += point_map.get(&e.0).unwrap() * e.1;
    }

    print!("Output: {}", sum);
}
