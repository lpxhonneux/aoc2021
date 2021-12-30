use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub enum Delim {
    Paran,
    Brack,
    Brace,
    Angle,
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./day10.txt") {
        let mut corrupt_lines = lines.map(|x| x.unwrap()
                                                .chars()
                                                .fold((Vec::new(), false), |acc, y|  {
                                                    if !acc.1 {
                                                        let corrupt = match *y {
                                                            '(' => {acc.0.push(*y); false},
                                                            '{' => {acc.0.push(*y); false},
                                                            '<' => {acc.0.push(*y); false},
                                                            '[' => {acc.0.push(*y); false},
                                                            y  => {if let Some(c) = acc.0.pop(); {
                                                                c == y
                                                            } else { true }
                                                            },
                                                        };
                                                        (acc.0, corrupt)
                                                    } else {
                                                        acc
                                                    }
                                                }
                                                )
                                          );
        println!("Day 10 part i: {}", 0);
        println!("Day 10 part ii: {:?}", 0);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
