use std::cmp::{max, min};
use std::fs;

fn print_field(field: &Vec<Vec<u32>>) {
    for line in field {
        for x in line {
            print!("{}", if *x == 0 {".".to_string()} else {(*x).to_string()});
        }
        println!();
    }
}

fn main() {
    let file_path = "inputs/input.5.txt";
    let contents = fs::read_to_string(file_path).expect(&format!("{} file not found", file_path));
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    let lines: Vec<((u32,u32),(u32,u32))> = contents.lines().map(|s| {
        let mut s = s.split(" -> ");
        let first: Vec<u32> = s.next().unwrap().split(",").map(|s| s.parse().expect("NaN")).collect();
        max_x = max(max(max_x, first[0] as usize), first[1] as usize);
        let point1 = (first[0], first[1]);
        let second: Vec<u32> = s.next().unwrap().split(",").map(|s| s.parse().expect("NaN")).collect();
        max_y = max(max(max_y, second[0] as usize), second[1] as usize);
        let point2 = (second[0], second[1]);
        return (point1, point2);
    }).collect();

    let mut field: Vec<Vec<u32>> = vec![vec![0; max_x+1]; max_y+1];

    println!("Test: {}", field[0][0]);

    for line in lines {
        let (x1, y1) = line.0;
        let (x2, y2) = line.1;

        if y1 == y2 {
            for x in min(x1,x2)..=max(x1, x2) {
                field[y1 as usize][x as usize] += 1;
            }
        } else if x1 == x2 {
            for y in min(y1,y2)..=max(y1, y2) {
                field[y as usize][x1 as usize] += 1;
            }
        }
    }

    //print_field(&field);

    let mut count = 0;

    for line in field {
        for x in line {
            if x >= 2 {
                count += 1;
            }
        }
    }



    print!("Output: {}", count);
}
