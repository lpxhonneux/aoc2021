use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Board {
    bingo: bool,
    board_numbers: Vec<Vec<u32>>,
    marked_pos: ([u32;5],[u32;5]),
    marked_num: [[bool;5];5],
}

impl Board {
    pub fn new(numbers : Vec<Vec<u32>>) -> Self {
        Board {
            bingo: false,
            board_numbers: numbers,
            marked_pos: ([0;5],[0;5]),
            marked_num: [[false;5];5],
        }
    }

    pub fn mark_number(&mut self, number: u32) {
        let mut row = 0;
        let mut col;
        for x in &self.board_numbers {
            col = 0;
            for y in x {
                if *y == number {
                    self.marked_pos.0[row] += 1;
                    self.marked_pos.1[col] += 1;
                    self.marked_num[row][col] = true;
                }
                col += 1;
            }
            row += 1;
        }
    }

    pub fn check_bingo(&mut self) -> bool {
        if self.bingo {
            return true;
        } else {
            for i in self.marked_pos.0.iter() { if *i == 5 { self.bingo = true; } }
            for j in self.marked_pos.1.iter() { if *j == 5 { self.bingo = true; } }
            return self.bingo
        }
    }

    pub fn unmarked_sum(&self) -> u32 {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                sum += if self.marked_num[i][j] { 0 } else { self.board_numbers[i][j] };
            }
        }
        return sum;
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(mut lines) = read_lines("./day4.txt") {
        let numbers = lines.nth(0).unwrap().unwrap().split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let mut boards = lines.map(|x| x.unwrap()).filter(|x| !(x.is_empty()))
                              .map(|x| x.split_whitespace()
                                        .map(|y| y.parse::<u32>().unwrap())
                                        .collect::<Vec<u32>>())
                              .collect::<Vec<Vec<u32>>>()
                              .chunks(5)
                              .map(|x| Board::new((*x).to_vec()))
                              .collect::<Vec<Board>>();
        let mut first_win = 0;
        let mut last_win = 0;
        for num in numbers {
            for b in &mut boards {
                if !b.check_bingo() {
                    b.mark_number(num);
                    if b.check_bingo() {
                        let sum_unmarked = b.unmarked_sum();
                        if first_win == 0 { first_win = num*sum_unmarked; }
                        last_win = num*sum_unmarked;
                    }
                }
            }
        }
        println!("Day 4 part i: {}", first_win);
        println!("Day 4 part ii: {}", last_win);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
