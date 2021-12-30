use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashSet;

pub fn hash(val: usize, size: usize) -> usize {
    val % size
}

pub fn coalesce(indices: Vec<(usize,usize)>, size: usize) -> Vec<(usize,usize)> {
    let mut hashset = vec![(false,0,0); indices.len()];
    indices.iter().map(|x| { hashset[hash(x.0 * size + x.1, size)] = (true, x.0, x.1); });
    hashset.iter().filter_map(|x| if x.0 { Some((x.1,x.2)) } else { None }).collect::<Vec<(usize,usize)>>()
}

pub fn flip(indices: (usize, usize), axis: char, value: usize) -> (usize, usize) {
    match axis {
        'x' => { (indices.0, if indices.1 > value { 2*value - indices.1 } else { indices.1 }) },
        _   => { (if indices.0 > value { 2*value - indices.0 } else { indices.0 }, indices.0) },
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(mut lines) = read_lines("./day13.txt") {
        let mut map = lines.take_while(|x| !x.unwrap().as_ref().is_empty())
                           .map(|x| x.unwrap())
                           .map(|x| {
                                let mut idx = x.split(' ');
                                (idx.nth(0).unwrap().parse::<usize>().unwrap(),
                                 idx.nth(0).unwrap().parse::<usize>().unwrap())
                           })
                           .collect::<Vec<(usize,usize)>>();
        let mut final_map = lines.map(|x| x.unwrap()).map(|x| x.split(' ').nth(2).unwrap()).map(|x| {
            let mut line = x.split('=');
            (line.nth(0).unwrap().chars().nth(0).unwrap(), line.nth(0).unwrap().parse::<usize>().unwrap())
        }).fold(map, |acc, x| acc.iter().map(|y| flip(*y, x.0, x.1)).collect::<Vec<(usize,usize)>>());
        final_map = coalesce(final_map, 1500);
        println!("Day 13 part i: {}", final_map.len());
        println!("Day 13 part ii: {:?}", 0);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
