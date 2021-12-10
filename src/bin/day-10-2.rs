use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "inputs/input.10.txt";
    let contents = fs::read_to_string(file_path).expect(&format!("{} file not found", file_path));
    let lines: Vec<_> = contents.lines().collect();

    let mut open_close: HashMap<char,char> = HashMap::new();
    open_close.insert('(', ')');
    open_close.insert('[', ']');
    open_close.insert('{', '}');
    open_close.insert('<', '>');

    let mut point_map: HashMap<char,u64> = HashMap::new();
    point_map.insert('(', 1);
    point_map.insert('[', 2);
    point_map.insert('{',  3);
    point_map.insert('<', 4);

    let mut scores: Vec<u64> = Vec::new();

    for line in lines {
        let mut char_stack: Vec<char> = Vec::new();

        let mut is_broken = false;
        for c in line.chars() {
            if open_close.contains_key(&c) {
                char_stack.push(c);
            } else {
                if let Some(last) = char_stack.pop() {
                    if c != *open_close.get(&last).unwrap() {
                        is_broken = true;
                        break;
                    }
                }
            }
        }
        if !is_broken {
            let mut sum = 0;
            for c in char_stack.into_iter().rev() {
                sum *= 5;
                sum += point_map.get(&c).unwrap();
            }
            scores.push(sum);
        }
    }

    scores.sort();
    print!("Output: {}", scores[scores.len() / 2]);
}
