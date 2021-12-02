use std::fs;

enum Direction {
    Forward,
    Up,
    Down,
}

impl Direction {
    fn from_str(input: &str) -> Result<Direction, ()> {
        match input {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

struct Movement {
    direction: Direction,
    units: u32,
}

struct Coordinates {
    horizontal: u32,
    depth: u32,
}

fn main() {
    let file_path = "inputs/input.2.txt";
    let contents = fs::read_to_string(file_path).expect(&format!("{} file not found", file_path));
    let movements: Vec<Movement> = contents.lines()
        .map(|s| s.split_whitespace())
        .map(|mut s| Movement {
            direction: Direction::from_str(s.next().unwrap()).unwrap(),
            units: s.next().unwrap().parse().unwrap(),
        }).collect();

    let mut coords = Coordinates{horizontal: 0, depth: 0};

    for movement in movements {
        match movement {
            Movement{direction: Direction::Forward, units: x} => coords.horizontal += x,
            Movement{direction: Direction::Up, units: x} => coords.depth -= x,
            Movement{direction: Direction::Down, units: x} => coords.depth += x,
        }
    }

    print!("Output: {}", coords.horizontal * coords.depth);
}
