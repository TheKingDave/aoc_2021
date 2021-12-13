use std::fs;
use itertools::Itertools;

fn main() {
    let file_path = "inputs/input.13.txt";
    let contents = fs::read_to_string(file_path).expect(&format!("{} file not found", file_path));

    let mut holes: Vec<(u32, u32)> = Vec::new();
    let mut folds: Vec<(char, u32)> = Vec::new();

    let mut collecting_holes = true;

    for line in contents.lines() {
        if line.is_empty() {
            collecting_holes = false;
            continue;
        }
        if collecting_holes {
            let mut split = line.splitn(2, ",");
            holes.push((split.next().unwrap().parse().unwrap(), split.next().unwrap().parse().unwrap()));
        } else {
            let xy = line.chars().skip(11).take(1).next().unwrap();
            let num = line.chars().skip(13).join("").parse().unwrap();

            folds.push((xy, num));
        }
    }

    for (dir, line) in folds {
        if dir == 'y' {
            for hole in holes.iter_mut() {
                if hole.1 > line {
                    hole.1 = line - (hole.1 - line)
                }
            }
        } else if dir == 'x' {
            for hole in holes.iter_mut() {
                if hole.0 > line {
                    hole.0 = line - (hole.0 - line)
                }
            }
        } else {
            panic!("Should not get here");
        }
        holes.sort_unstable();
        holes.dedup();
    }

    // EPLGRULR
    for i in 0..7 {
        for a in 0..40 {
            print!("{}", if holes.contains(&(a, i)) {'#'} else {'.'});
        }
        println!();
    }
}
