use std::collections::HashSet;
use std::fs;
use itertools::Itertools;

fn main() {
    let file_path = "inputs/input.9.txt";
    let contents = fs::read_to_string(file_path).expect(&format!("{} file not found", file_path));

    let grid: Vec<Vec<u8>> = contents.lines().map(|s| s.chars().map(|x| x.to_digit(10).unwrap() as u8).collect()).collect();

    let i_len = grid.len();
    let a_len = grid.first().unwrap().len();

    let mut lows: Vec<(usize, usize)> = Vec::new();

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
                lows.push((i, a));
            }
        }
    }

    let basins: Vec<usize> = lows.iter().map(|loc| basin_size_calc(&grid, loc)).collect();

    let sum: usize = basins.iter().sorted().rev().take(3).product();

    print!("Output: {}", sum);
}

fn basin_size_calc(grid: &Vec<Vec<u8>>, start: &(usize, usize)) -> usize {
    let visited: &mut HashSet<(usize, usize)> = &mut HashSet::new();

    basin_size(grid, *start, visited);

    (*visited).len()
}

fn basin_size(grid: &Vec<Vec<u8>>, start: (usize, usize), visited: &mut HashSet<(usize, usize)>) {
    let (i, a) = start;
    let element = *grid.get(i).unwrap().get(a).unwrap();
    if element == 9 {
        return;
    }
    if !visited.insert(start) {
        return;
    }

    if i != 0 {basin_size(grid, (i-1, a), visited)}
    if i+1 < grid.len() {basin_size(grid, (i+1, a), visited)}

    if a != 0 {basin_size(grid, (i, a-1), visited)}
    if a+1 < grid.first().unwrap().len() {basin_size(grid, (i, a+1), visited)}
}