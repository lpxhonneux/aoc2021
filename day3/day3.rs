use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Node {
    zero: (u32, Option<Box<Node>>),
    one: (u32, Option<Box<Node>>),
}

impl Node {
    pub fn new() -> Self {
        Node { zero: (0,None), one: (0,None) }
    }

    pub fn insert(&mut self, mut x: Vec<char>) {
        if let Some(c) = x.pop() {
            let mut branch;
            match c {
                '0' => {
                    branch = &mut self.zero;
                },
                '1' => {
                    branch = &mut self.one;
                },
                _ => {
                    branch = &mut self.zero;
                }
            };
            branch.0 += 1;
            let branch_link = &mut branch.1;
            match branch_link {
                Some(branch_link) => branch_link.insert(x),
                None => {
                    let mut branch_link = Box::new(Node::new());
                    branch_link.insert(x);
                    branch.1 = Some(branch_link);
                },
            }
        }
    }

    pub fn find_max(&self, path: u32) -> u32 {
        if self.zero.0 == 0 && self.one.0 == 0 {
            return path;
        } else if self.zero.0 > self.one.0 {
            if let Some(next_node) = &self.zero.1 {
                return next_node.find_max(path << 1)
            }
            return path;
        } else {
            if let Some(next_node) = &self.one.1 {
                return next_node.find_max((path << 1) + 1)
            }
            return path;
        }
    }

    pub fn find_min(&self, path: u32) -> u32 {
        if self.zero.0 == 0 && self.one.0 == 0 {
            return path;
        } else if (self.zero.0 > self.one.0 && self.one.0 != 0) || self.zero.0 == 0 {
            if let Some(next_node) = &self.one.1 {
                return next_node.find_min((path << 1) + 1)
            }
            return path;
        } else {
            if let Some(next_node) = &self.zero.1 {
                return next_node.find_min(path << 1)
            }
            return path;
        }
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./day3.txt") {
        let counts = lines.map(|x| x.unwrap().chars().collect::<Vec<char>>() )
            .fold([0;12], |mut cnt, x|
                  {
                      for (i,c) in x.into_iter().enumerate() {
                          if c == '0' {
                              cnt[i] = cnt[i]-1;
                          } else if c == '1' {
                              cnt[i] = cnt[i]+1;
                          }
                      }
                  cnt
                          }
                  );
        let (gamma, epsilon) = counts.iter()
            .fold((0,0), |(mcb, lcb), x|
                if x > &0 {
                    ((mcb << 1) + 1, lcb << 1)
                } else {
                    (mcb << 1, (lcb << 1) + 1)
                }
            );
        println!("Day 3 part i: {}", gamma*epsilon);
    }
    if let Ok(lines) = read_lines("./day3.txt") {
        let mut root = Node::new();
        for num in lines.map(|x| x.unwrap().chars().rev().collect::<Vec<char>>() ) {
            root.insert(num);
        }
        let o2 = root.find_max(0);
        let co2 = root.find_min(0);
        println!("Day 3 part ii:{}", o2*co2);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
