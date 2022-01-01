use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::max;

use std::collections::HashSet;

pub fn coalesce(indices: Vec<(usize,usize)>) -> Vec<(usize,usize)> {
    let mut set_idx = HashSet::new();
    indices.iter().for_each(|x| { set_idx.insert((x.0, x.1)); });
    set_idx.drain().collect::<Vec<(usize,usize)>>()
}

pub fn flip(indices: (usize, usize), axis: char, value: usize) -> (usize, usize) {
    match axis {
        'y' => { (indices.0, if indices.1 >= value { 2*value - indices.1 } else { indices.1 }) },
        _   => { (if indices.0 >= value { 2*value - indices.0 } else { indices.0 }, indices.1) },
    }
}

pub fn visualise(indices: Vec<(usize,usize)>) {
    let (max_x, max_y) = indices.iter().fold((0,0), |acc, (x,y)| (max(acc.0,*x),max(acc.1,*y)) );
    let mut canvas = vec![vec!['.';max_x+1];max_y+1];
    for (i,j) in indices {
        canvas[j][i] = '#';
    }
    for line in canvas {
        let s : String = line.iter().collect();
        println!("{}", s);
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(mut lines) = read_lines("./day13.txt") {
        let map = lines.by_ref()
                           .take_while(|x:&Result<String, std::io::Error>| !x.as_ref().unwrap().is_empty())
                           .map(|x| x.unwrap())
                           .map(|x| {
                                let mut idx = x.split(',');
                                (idx.nth(0).unwrap().parse::<usize>().unwrap(),
                                 idx.nth(0).unwrap().parse::<usize>().unwrap())
                           })
                           .collect::<Vec<(usize,usize)>>();
        let mut final_map = lines.map(|x| x.unwrap())
                                 .map(|x| {
                                     let mut s = x.split(' ');
                                     let t = s.nth(2).unwrap();
                                     let mut line = t.split('=');
                                    (line.nth(0).unwrap().chars().nth(0).unwrap(),
                                     line.nth(0).unwrap().parse::<usize>().unwrap())
                                    }
                                 ).fold(map,
                                        |acc, x| acc.iter().map(|y| flip(*y, x.0, x.1)).collect::<Vec<(usize,usize)>>());
        final_map = coalesce(final_map);
        // Getting the solutions is a bit adhoc for part 1 simply remove the all the unnecessary fold instructions from the file
        // For part 2 we print it our to screen and then visually recognize the code
        println!("Day 13 part number of elements: {}", final_map.len());
        println!("Day 13 part visual");
        visualise(final_map);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
