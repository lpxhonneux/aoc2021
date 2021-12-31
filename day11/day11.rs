use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone)]
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

    pub fn new_from_const(n_col: usize, n_row: usize, value: T) -> Self {
        Vec2d{
            n_col: n_col,
            array: vec![value;n_col*n_row]
        }
    }

    pub fn merge(&mut self, new_row: &mut Vec2d<T>) {
        self.array.append(&mut new_row.array);
    }

    pub fn index(&self, i:i32, j:i32) -> Option<T> {
        if i < 0 || j < 0 || j >= self.n_col as i32 {
            None
        } else {
            let hidden_idx = self.n_col as i32 * i + j;
            if hidden_idx >= self.size() as i32 || hidden_idx < 0 {
                None
            } else {
                Some(self.array[hidden_idx as usize])
            }
        }
    }

    pub fn size(&self) -> usize {
        self.array.len()
    }

    pub fn height_width(&self) -> (usize, usize) {
        (self.size()/self.n_col, self.n_col)
    }

    pub fn set_index(&mut self, i:i32, j:i32, num:T) {
        if i < 0 || j < 0 || j >= self.n_col as i32 {
            ()
        } else {
            let hidden_idx = self.n_col as i32 * i + j;
            if !(hidden_idx >= self.size() as i32 || hidden_idx < 0) {
                self.array[hidden_idx as usize] = num;
            }
        }
    }

}

pub fn equal<T>(a: &Vec2d<T>, b: &Vec2d<T>) -> bool
    where T: std::cmp::PartialEq
{
    a.array.iter().zip(b.array.iter()).all(|(x,y)| *x==*y)
}

pub fn addconst(map: &mut Vec2d<u32>, num: u32) {
    map.array = map.array.iter().map(|x| x+num).collect::<Vec<u32>>();
}

fn main() {
    // File hosts must exist in current path before this produces output
    let adj: Vec<(i32,i32)> = vec![(1,1), (1,0), (0,1), (1,-1), (-1,-1),(-1,0),(0,-1),(-1,1)];
    if let Ok(lines) = read_lines("./day11.txt") {
        let mut map = lines.map(|x| x.unwrap()
                            .chars()
                            .map(|y| y.to_digit(10).unwrap())
                            .collect::<Vec<u32>>()
        ).map(|x| Vec2d::new_from_vec(x))
         .reduce(|mut acc, mut x|  {acc.merge(&mut x); acc }).unwrap();
        let (nrow, ncol) = map.height_width();
        let mut flash_counter = 0;
        let mut flash_counter_100 = 0;
        let mut all_flashed_first_time = -1;
        for t in 0..1000 {
            let mut flashed = Vec2d::new_from_const(ncol, nrow, false);
            addconst(&mut map,1);
            let mut flashes_this_iter = 0;
            loop {
                let prev_map = map.clone();
                for i in 0..(nrow as i32) {
                    for j in 0..(ncol as i32) {
                        if let Some(false) = flashed.index(i,j) {
                            if prev_map.index(i,j).unwrap() > 9 {
                                flashed.set_index(i,j,true);
                                flashes_this_iter += 1;
                                map.set_index(i,j,0);
                                for (k1,k2) in &adj {
                                    if let Some(false) = flashed.index(i+*k1, j+*k2) {
                                        map.set_index(i+*k1, j+*k2, map.index(i+*k1,j+*k2).unwrap()+1);
                                    }
                                }
                            }
                        }
                    }
                }
                // println!("Flashed {:?}", flashed.array);
                // println!("Map {:?}", map.array);
                // println!("Prev Map {:?}", prev_map.array);
                if equal(&prev_map, &map) {
                    break;
                }
            }
            flash_counter += flashes_this_iter;
            if t == 99 {
                flash_counter_100 = flash_counter;
            }
            if flashes_this_iter >= nrow*ncol && all_flashed_first_time < 0 {
                all_flashed_first_time = t+1;
            }
            if t > 99 && all_flashed_first_time >= 0 {
                break;
            }
            // println!("Map {:?}", map.array);
        }
        println!("Day 11 part i: {}", flash_counter_100);
        println!("Day 11 part ii: {:?}", all_flashed_first_time);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
