use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Command {
    Forward,
    Down,
    Up,
    NOP,
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./day2.txt") {
        let (final_h, final_d) = lines.map(|x| {
            let x_value = x.unwrap();
            let mut x_split = x_value.split_whitespace();
            (match x_split.next().unwrap() {
                "forward" => Command::Forward,
                "down" => Command::Down,
                "up" => Command::Up,
                _ => Command::NOP
            }, x_split.next().unwrap().parse::<i32>().unwrap())
        })
                                    .fold((0,0), |(h,d), (cmd, val)|
                                          match cmd {
                                              Command::Forward => (h+val, d),
                                              Command::Down => (h, d+val),
                                              Command::Up => (h, d-val),
                                              Command::NOP => (h,d)
                                          }
                                          );
        println!("Day 2 part i: {}", final_h*final_d);
    }
    if let Ok(lines) = read_lines("./day2.txt") {
        let (final_h, final_d, _aim) = lines.map(|x| {
            let x_value = x.unwrap();
            let mut x_split = x_value.split_whitespace();
            (match x_split.next().unwrap() {
                "forward" => Command::Forward,
                "down" => Command::Down,
                "up" => Command::Up,
                _ => Command::NOP
            }, x_split.next().unwrap().parse::<i32>().unwrap())
        })
                                    .fold((0,0,0), |(h,d, aim), (cmd, val)|
                                          match cmd {
                                              Command::Forward => (h+val, d + aim*val, aim),
                                              Command::Down => (h, d, aim+val),
                                              Command::Up => (h, d, aim-val),
                                              Command::NOP => (h,d, aim)
                                          }
                                          );
        println!("Day 2 part ii: {}", final_h*final_d)
        // Consumes the iterator, returns an (Optional) String
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
