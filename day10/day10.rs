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
        let all_lines = lines.map(|x| x.unwrap()
                                                .chars()
                                                .fold((Vec::new(), false), |mut acc, y|  {
                                                    if !acc.1 {
                                                        let corrupt = match y {
                                                            '(' => {acc.0.push(y); false},
                                                            '{' => {acc.0.push(y); false},
                                                            '<' => {acc.0.push(y); false},
                                                            '[' => {acc.0.push(y); false},
                                                            '}'  => {
                                                                if let Some(c) = acc.0.pop() {
                                                                    c != '{'
                                                                } else { true }
                                                            },
                                                            ')'  => {
                                                                if let Some(c) = acc.0.pop() {
                                                                    c != '('
                                                                } else { true }
                                                            },
                                                            ']'  => {
                                                                if let Some(c) = acc.0.pop() {
                                                                    c != '['
                                                                } else { true }
                                                            },
                                                            '>'  => {
                                                                if let Some(c) = acc.0.pop() {
                                                                    c != '<'
                                                                } else { true }
                                                            },
                                                            _    => false,
                                                        };
                                                        if corrupt {
                                                            acc.0.push(y);
                                                        }
                                                        (acc.0, corrupt)
                                                    } else {
                                                        acc
                                                    }
                                                }
                                                )
                                          );
        let (uncorrupted_lines, corrupt_lines) : (Vec<(Vec<char>, bool)>,Vec<(Vec<char>, bool)>) = all_lines.partition(|x| !x.1);
        println!("Day 10 part i: {}", corrupt_lines.into_iter().fold(0, |acc, mut x| {
                                              if x.1 {
                                                  if let Some(c) = x.0.pop() {
                                                      match c {
                                                          ')' => acc+3,
                                                          ']' => acc+57,
                                                          '}' => acc+1197,
                                                          '>' => acc+25137,
                                                          _   => acc+0,
                                                      }
                                                  } else {
                                                      acc
                                                  }
                                              } else {
                                                  acc
                                              }
                                          }));
        let mut missing_score = uncorrupted_lines.into_iter()
            .map(|x| x.0)
            .map(|x| {
                x.iter().rev().fold(0 as u64, |acc, y| {
                    match y {
                        '(' => {acc*5+1},
                        '{' => {acc*5+3},
                        '<' => {acc*5+4},
                        '[' => {acc*5+2},
                        _   => {acc},
                    }
                })
            }).collect::<Vec<u64>>();
        missing_score.sort();
        println!("Day 10 part ii: {:?}", missing_score[missing_score.len()/2]);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
