use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::cmp::min;

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

    pub fn set_index(&mut self, i:i32, j:i32, num:T) -> T {
        if i < 0 || j < 0 || j >= self.n_col as i32 {
            ()
        } else {
            let hidden_idx = self.n_col as i32 * i + j;
            if !(hidden_idx >= self.size() as i32 || hidden_idx < 0) {
                self.array[hidden_idx as usize] = num;
            }
        }
        num
    }

}

pub fn min_dyn_risk(risk_map: &Vec2d<u32>, total_risk: &mut Vec2d<u32>, i: i32, j: i32) -> u32 {
    let (nrow,ncol) = risk_map.height_width();
    if i < 0 || j < 0 || i >= (nrow as i32) || j >= (ncol as i32) {
        u32::MAX
    } else if i == 0 && j == 0 {
        0
    } else if let Some(c) = total_risk.index(i,j) {
        if c == u32::MAX {
            let risk = risk_map.index(i,j).unwrap_or(0);
            let risk_left = min_dyn_risk(risk_map, total_risk, i-1, j);
            let risk_up = min_dyn_risk(risk_map, total_risk, i, j-1);
            total_risk.set_index(i,j,risk + min(risk_left,risk_up))
        } else {
            c
        }
    } else {
        u32::MAX
    }
}

pub fn optimise_risk(risk_map: &Vec2d<u32>, total_risk: &mut Vec2d<u32>) {
    let mut change_this_iter;
    let (nrow, ncol) = total_risk.height_width();
    let adj = vec![(1,0), (0,1), (-1,0), (0,-1)];
    loop {
        change_this_iter = false;
        for i in 0..nrow as i32 {
            for j in 0..ncol as i32 {
                let mut min_val = total_risk.index(i,j).unwrap_or(u32::MAX)-risk_map.index(i,j).unwrap_or(0);
                for (k1,k2) in &adj {
                    if let Some(adj_val) = total_risk.index(i+k1,j+k2) {
                        if adj_val < min_val {
                            change_this_iter = true;
                            min_val = adj_val;
                        }
                    }
                }
                total_risk.set_index(i,j, min_val + risk_map.index(i,j).unwrap_or(0));
            }
        }
        if !change_this_iter {
            break;
        }
    }
}

pub fn fivexfive(map: &mut Vec2d<u32>) {
    let (nrow,ncol) = map.height_width();
    map.array = map.array.chunks(ncol)
        .map(|x| x.iter().cycle().take(ncol*5).cloned().collect::<Vec<u32>>())
        .cycle()
        .take(5*nrow)
        .reduce(|mut acc, mut x|  {acc.append(&mut x); acc }).unwrap();
    map.n_col = 5*ncol;
    map.array = map.array.iter()
        .enumerate()
        .map(|(ith,x)| {
            let (tile_row, tile_col) = (ith/(5*ncol*nrow), (ith % (5*ncol))/ncol);
            let new_x = x+tile_row as u32 +tile_col as u32;
            if new_x > 9 { (new_x % 10) + 1 } else { new_x }
        })
        .collect();
}

fn main() {
    // File hosts must exist in current path before this produces output
    println!("DAY 15");
    if let Ok(lines) = read_lines("./day15.txt") {
        let mut map = lines.map(|x| x.unwrap()
                            .chars()
                            .map(|y| y.to_digit(10).unwrap())
                            .collect::<Vec<u32>>()
        ).map(|x| Vec2d::new_from_vec(x))
         .reduce(|mut acc, mut x|  {acc.merge(&mut x); acc }).unwrap();
        let (nrow, ncol) = map.height_width();
        let mut total_risk = Vec2d::new_from_const(ncol, nrow, u32::MAX);
        let _estimate_risk = min_dyn_risk(&map, &mut total_risk, nrow as i32 - 1, ncol as i32 - 1);
        optimise_risk(&map, &mut total_risk);
        println!("Part i: {}", total_risk.index(nrow as i32 -1, ncol as i32 - 1).unwrap_or(0));
        fivexfive(&mut map);
        total_risk = Vec2d::new_from_const(5*ncol, 5*nrow, u32::MAX);
        min_dyn_risk(&map, &mut total_risk, 5*nrow as i32 - 1, 5*ncol as i32 - 1);
        optimise_risk(&map, &mut total_risk);
        println!("Part ii: {:?}", total_risk.index(5*nrow as i32 -1, 5*ncol as i32 -1).unwrap_or(0));
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
