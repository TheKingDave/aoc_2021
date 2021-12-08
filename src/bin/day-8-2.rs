use std::collections::HashMap;
use std::fs;
use itertools::Itertools;

fn one_diff(a: &String, b: &String) -> char {
    if a.len() == b.len() {panic!("String are the same length")}
    if a.len() > b.len() {
        for c in a.chars() {
            if !b.contains(c) {
                return c;
            }
        }
    } else {
        for c in b.chars() {
            if !a.contains(c) {
                return c;
            }
        }
    }
    panic!("Could not find one difference {}, {}", a, b);
}

fn missing(a: &String, b: &String) -> char {
    for c in a.chars() {
        if !b.contains(c) {
            return c;
        }
    }
    panic!("Could not find missing between {}, {}", a, b);
}

fn contains_chars(a: &String, search: &String) -> bool {
    for c in search.chars() {
        if !a.contains(c) {
            return false;
        }
    }
    true
}

fn one_len(p: &Vec<&str>, len: usize) -> String {
    String::from(*p.iter().find(|x| x.len() == len).unwrap())
}

fn all_len(p: &Vec<&str>, len: usize) -> Vec<String> {
    (*p.iter().filter(|x| x.len() == len).map(|s| String::from(*s)).collect::<Vec<String>>()).to_vec()
}

fn sort_str(s: &String) -> String {
    s.chars().sorted().rev().collect::<String>()
}

fn main() {
    let file_path = "inputs/input.8.txt";
    let contents = fs::read_to_string(file_path).expect(&format!("{} file not found", file_path));

    let mut digits: Vec<(Vec<&str>, Vec<&str>)> = Vec::new();

    for line in contents.trim().lines() {
        let mut parts = line.split("|").map(|x| x.trim());

        digits.push((
            parts.next().unwrap().split_whitespace().collect(),
            parts.next().unwrap().split_whitespace().collect()
        ));
    }

    let mut sum = 0;

    for line in digits {
        let (patterns, searched) = line;

        let mut mapping: HashMap<&String, u8> = HashMap::new();
        // Should, Is
        let mut segments: HashMap<char, char> = HashMap::new();

        let one = one_len(&patterns, 2);
        let four = one_len(&patterns, 4);
        let seven = one_len(&patterns, 3);
        let eight = one_len(&patterns, 7);

        mapping.insert(&one, 1);
        mapping.insert(&four, 4);
        mapping.insert(&seven, 7);
        mapping.insert(&eight, 8);

        // Find a
        let a = one_diff(&one, &seven);
        segments.insert('a', a);

        // println!("{:?} {}", all_len(&patterns, 5), one);
        let mut unknown_five = all_len(&patterns, 5);
        let three = String::from(unknown_five.iter().find(|p| contains_chars(p, &one)).unwrap());
        mapping.insert(&three, 3);
        unknown_five.remove(unknown_five.iter().position(|s| s.eq(&three)).unwrap());


        let filter_g = &mut four.clone();
        filter_g.push_str(&seven);
        let g = missing(&three, filter_g);
        segments.insert('g', g);

        let filter_d = &mut one.clone();
        filter_d.push(*segments.get(&'a').unwrap());
        filter_d.push(*segments.get(&'g').unwrap());
        let d = missing(&three, filter_d);
        segments.insert('d', d);

        let mut unknown_six = all_len(&patterns, 6);

        let zero = String::from(unknown_six.iter().find(|p| !p.contains(*segments.get(&'d').unwrap())).unwrap());
        mapping.insert(&zero, 0);
        unknown_six.remove(unknown_six.iter().position(|s| s.eq(&zero)).unwrap());


        let mut a = 0;
        let mut b = 0;
        for p in &unknown_six {
            if p.contains(one.chars().next().unwrap()) {
                a += 1;
            }
            if p.contains(one.chars().skip(1).next().unwrap()) {
                b += 1;
            }
        }
        if b > a {
            segments.insert('c', one.chars().next().unwrap());
        } else {
            segments.insert('c', one.chars().skip(1).next().unwrap());
        }
        segments.insert('f', missing(&one, &String::from(*segments.get(&'c').unwrap())));


        let first_9 = unknown_six.iter().next().unwrap().contains(*segments.get(&'c').unwrap());
        let mut iter = unknown_six.iter();
        mapping.insert(iter.next().unwrap(), if first_9 {9} else {6});
        mapping.insert(iter.next().unwrap(), if first_9 {6} else {9});

        let first_2 = unknown_five.iter().next().unwrap().contains(*segments.get(&'c').unwrap());
        let mut iter = unknown_five.iter();
        mapping.insert(iter.next().unwrap(), if first_2 {2} else {5});
        mapping.insert(iter.next().unwrap(), if first_2 {5} else {2});

        let mut sorted_mapping = HashMap::new();
        for (key, value) in mapping {
            sorted_mapping.insert(sort_str(key), value);
        }

        let mut number: u32 = 0;
        for (i, num) in searched.iter().rev().enumerate() {
            if let Some(x) = sorted_mapping.get(&sort_str(&String::from(*num))) {
                number += (*x as u32) * 10_u32.pow(i as u32);
            } else {
                panic!("Could not find mapping {} in {:?}", num, sorted_mapping);
            }
        }
        sum += number;
    }

    print!("Output: {}", sum);
}
