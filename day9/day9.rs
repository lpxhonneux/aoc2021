use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Vec2d<T> {
    n_col: usize,
    array: Vec<T>
}

impl<T: std::clone::Clone + std::cmp::PartialOrd + std::marker::Copy> Vec2d<T> {
    pub fn new_from_vec(first_row: Vec<T>) -> Self {
        Vec2d {
            n_col: first_row.len(),
            array: first_row
        }
    }

    pub fn new(n_col: usize) -> Self {
        Vec2d {
            n_col,
            array: Vec::new()
        }
    }

    pub fn merge(&mut self, new_row: &mut Vec2d<T>) {
        self.array.append(&mut new_row.array);
    }

    pub fn index(&self, i:i32, j:i32) -> Option<T> {
        let hidden_idx = self.n_col as i32 * i + j;
        if hidden_idx >= self.size() as i32 || hidden_idx < 0 {
            None
        } else {
            Some(self.array[hidden_idx as usize])
        }
    }

    pub fn size(&self) -> usize {
        self.array.len()
    }

    pub fn cmp_to_adj_min(&self, i:i32, j:i32) -> (bool, T) {
        let val = self.index(i, j).unwrap();
        let adj = [self.index(i+1, j),
                   self.index(i, j+1),
                   self.index(i-1, j),
                   self.index(i, j-1)
        ];
        (adj.into_iter().all(|x| match x {
                Some(x) => x > &val,
                None    => true,
            }
        ), val)
    }
}

pub enum LineRelation {
    DisjointLeft,
    DisjointRight,
    Contains,
    IsContained,
    EndsFirst,
    EndsLast
}

type Basin = Vec<(u32,u32,u32)>;
type Boundary = Vec<(u32,u32,u32)>;
type ActiveBasin = (Basin, Boundary);

pub fn split_row(row: Vec<u32>) -> Boundary
{
    let len = row.len();
    let mut result = Vec::new();
    let mut last_i = 0;
    for (i,h) in row.iter().enumerate() {
        if *h == 9 {
            if i != last_i {
                result.push((last_i as u32, i as u32, (i-last_i) as u32));
            }
            last_i = i+1;
        }
    }
    if len != last_i {
        result.push((last_i as u32, len as u32, (len-last_i) as u32));
    }
    result
}

pub fn compare_lines(line1: (u32,u32,u32), line2: (u32,u32,u32)) -> LineRelation {
    if line1.0 >= line2.1 {
        LineRelation::DisjointRight
    } else if line2.0 >= line1.1 {
        LineRelation::DisjointLeft
    } else {
        if line1.0 <= line2.0 {
            if line1.1 >= line2.1 {
                LineRelation::Contains
            } else {
                LineRelation::EndsFirst
            }
        } else {
            if line1.1 <= line2.1 {
                LineRelation::IsContained
            } else {
                LineRelation::EndsLast
            }
        }
    }
}

pub fn check_overlap(basin: Option<ActiveBasin>, bound: Option<(u32,u32,u32)>) -> LineRelation {
    match (basin,bound) {
        (None, _) => LineRelation::DisjointRight,
        (_, None) => LineRelation::DisjointLeft,
        (Some((_,basin)), Some(bound)) => {
            let relation = basin.iter().fold(
                (false, false, false),
                |mut acc, x|  {
                    match compare_lines(*x, bound) {
                        LineRelation::DisjointLeft => { acc.0 = true; },
                        LineRelation::DisjointRight => { acc.2 = true; },
                        LineRelation::EndsFirst => { acc.1 = true; acc.0 = true; },
                        LineRelation::EndsLast => { acc.1 = true; acc.2 = true; },
                        LineRelation::IsContained => { acc.1 = true; },
                        LineRelation::Contains => { acc.0 = true; acc.1 = true; acc.2 = true; },
                    };
                    acc
                }
            );
            match relation {
                (_, false, false) => LineRelation::DisjointLeft,
                (_, false, true) => LineRelation::DisjointRight,
                (true, true, false) => LineRelation::EndsFirst,
                (true, true, true) => LineRelation::Contains,
                (false, true, false) => LineRelation::IsContained,
                (_, true, true) => LineRelation::EndsLast,
            }
        }
    }
}

pub fn count_basins(twodarray: &mut Vec2d<u32>) -> Vec<u32> {
    let basin_rows = twodarray.array.chunks(twodarray.n_col).map(|r| split_row(r.to_vec()));
    let basins :(Vec<Basin>,Vec<ActiveBasin>) = basin_rows.fold(
        (Vec::new(), Vec::new()),
        |mut acc: (Vec<Basin>,Vec<ActiveBasin>), x: Boundary| {
            let mut cur_basin = (Vec::new(), Vec::new());
            let mut completed_basins  = acc.0;
            let mut alive_basins = Vec::new();
            let mut active_basin_iter = acc.1.iter();
            let mut active_basin = active_basin_iter.next();
            let mut active_boundary_iter = x.iter();
            let mut active_boundary = active_boundary_iter.next();
            loop {
                if active_basin != None || active_boundary != None {
                    match check_overlap(active_basin.cloned(), active_boundary.copied()) {
                        LineRelation::DisjointLeft  => { cur_basin.0.append(&mut active_basin.unwrap().0.clone());
                                                         cur_basin.0.append(&mut active_basin.unwrap().1.clone());
                                                         if cur_basin.1.len() == 0 {
                                                            completed_basins.push(cur_basin.0);
                                                         } else {
                                                             alive_basins.push(cur_basin);
                                                         }
                                                         active_basin = active_basin_iter.next();
                                                         cur_basin = (Vec::new(), Vec::new());
                        },
                        LineRelation::DisjointRight => { cur_basin.1.push(*active_boundary.unwrap());
                                                         alive_basins.push(cur_basin);
                                                         active_boundary = active_boundary_iter.next();
                                                         cur_basin = (Vec::new(), Vec::new());
                        },
                        LineRelation::Contains      => { cur_basin.1.push(*active_boundary.unwrap());
                                                         active_boundary = active_boundary_iter.next();
                        },
                        LineRelation::IsContained   => { cur_basin.0.append(&mut active_basin.unwrap().0.clone());
                                                         cur_basin.0.append(&mut active_basin.unwrap().1.clone());
                                                         active_basin = active_basin_iter.next();
                        },
                        LineRelation::EndsFirst     => { cur_basin.0.append(&mut active_basin.unwrap().0.clone());
                                                         cur_basin.0.append(&mut active_basin.unwrap().1.clone());
                                                         active_basin = active_basin_iter.next();
                        },
                        LineRelation::EndsLast      => { cur_basin.1.push(*active_boundary.unwrap());
                                                         active_boundary = active_boundary_iter.next();
                        },
                    }
                } else {
                    break;
                }

            }
        (completed_basins, alive_basins)
    });
    let mut all_basins = basins.0;
    for active in basins.1 {
        let mut finished_basin = active.0;
        finished_basin.append(&mut active.1.clone());
        all_basins.push(finished_basin);
    }
    all_basins.iter().map(|x| x.iter().fold(0, |acc,y| acc+y.2)).collect::<Vec<u32>>()
}

fn top3(cur_top3: (u32,u32,u32), next: u32) -> (u32,u32,u32) {
    if next > cur_top3.0 {
        (next, cur_top3.0, cur_top3.1)
    } else if next > cur_top3.1 {
        (cur_top3.0, next, cur_top3.1)
    } else if next > cur_top3.2 {
        (cur_top3.0, cur_top3.1, next)
    } else {
        cur_top3
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./day9.txt") {
        let mut map = lines.map(|x| x.unwrap()
                            .chars()
                            .map(|y| y.to_digit(10).unwrap())
                            .collect::<Vec<u32>>()
        ).map(|x| Vec2d::new_from_vec(x))
         .reduce(|mut acc, mut x|  {acc.merge(&mut x); acc }).unwrap();
        let risk = (0..map.size()).map(|i| {
            let (q,r) = (i / map.n_col, i% map.n_col);
            map.cmp_to_adj_min(q as i32, r as i32)
        }).filter_map(|(b, val)| if b { Some(val + 1) } else { None });
        println!("Day 9 part i: {}", risk.sum::<u32>());
        let basin_sizes = count_basins(&mut map);
        let (a,b,c) = basin_sizes.iter().fold((0,0,0), |acc, x| top3(acc, *x));
        println!("Day 9 part ii: {:?}", a*b*c);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
