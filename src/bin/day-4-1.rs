use std::fs;

#[derive(Clone, Copy)]
struct Board {
    numbers: [[u32; 5]; 5],
    marked: [[bool; 5]; 5],
}

impl Board {
    fn new(numbers: [[u32; 5]; 5]) -> Board {
        Board {
            numbers,
            marked: [[false; 5]; 5],
        }
    }

    fn mark(&mut self, number: u32) {
        for i in 0..5 {
            for a in 0..5 {
                if self.numbers[i][a] == number {
                    self.marked[i][a] = true;
                }
            }
        }
    }

    fn wins(&self) -> bool {
        for row in self.marked {
            if row.iter().all(|x| *x) {
                return true;
            }
        }
        for index in 0..5 {
            let mut _marked = [false; 5];
            for row in 0..5 {
                _marked[row] = self.marked[row][index];
            }
            if _marked.iter().all(|x| *x) {
                return true;
            }
        }
        false
    }

    fn not_marked(&self) -> Vec<u32> {
        let mut nums = Vec::new();
        for i in 0..5 {
            for a in 0..5 {
                if !self.marked[i][a] {
                    nums.push(self.numbers[i][a])
                }
            }
        }
        nums
    }

    fn print(&self) {
        for i in 0..5 {
            for a in 0..5 {
                print!("{:2}{} ", self.numbers[i][a], if self.marked[i][a] {"M"} else {" "});
            }
            println!();
        }
        println!();
    }
}

fn main() {
    let file_path = "inputs/input.4.txt";
    let contents = fs::read_to_string(file_path).expect(&format!("{} file not found", file_path));
    let mut lines: Vec<&str> = contents.lines().collect();

    let draw: Vec<u32> = lines.first().unwrap().split(",").map(|x| x.parse().expect("NaN 1")).collect();
    lines.drain(..1);

    let mut boards: Vec<Board> = Vec::new();

    while !lines.is_empty() {
        lines.drain(..1);
        let board = lines.drain(..5);
        let mut numbers: [[u32; 5]; 5] = [[0; 5]; 5];
        for (index, line) in board.enumerate() {
            let split: Vec<u32> = line.split_whitespace().map(|x| x.parse().expect("NaN 2")).collect();
            if split.len() != 5 { panic!("Split is not 5 long") }

            for i in 0..5 {
                numbers[index][i] = split[i];
            }
        }
        let board = Board::new(numbers);
        board.print();
        let _ = &boards.push(board);
    }

    for num in draw {
        for board in &mut boards {
            board.mark(num);
            //board.print();
            if board.wins() {
                let sum: u32 = board.not_marked().into_iter().sum();
                println!("Output: {}", sum * num);
                return;
            }
        }
    }


    println!("Output: {}", false);
}
