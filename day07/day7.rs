use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;

fn new_fuel_cost(x: i32, m: i32) -> i32 {
    (x-m).abs()*((x-m).abs()+1)
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(mut lines) = read_lines("./day7.txt") {
        let mut positions = lines.nth(0)
                        .unwrap().unwrap()
                        .split(',')
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();
        positions.sort_unstable();
        let median_index = (positions.len() / 2, positions.len() % 2);
        let median = if median_index.1 == 0 {
            positions[median_index.0] as f64
        } else {
            ((positions[median_index.0] + positions[median_index.0]) as f64)/2.0
        };
        let fuel = positions.iter().fold(0.0, |acc, x| acc + (*x as f64 -median).abs());
        println!("Day 5 part i: {}", fuel);
        let mean = ((positions.iter().sum::<u32>() as f64)/(positions.len() as f64)).round() as i32;
        let possible_fuel = positions.iter()
                                     .fold(
                                         (0,0,0),
                                         |(acc0, acc1, acc2),x| (acc0 + new_fuel_cost(*x as i32, mean-1),
                                                                acc1 + new_fuel_cost(*x as i32, mean),
                                                                acc2 + new_fuel_cost(*x as i32, mean+1)
                                                                )
                                         );
        let best_fuel = cmp::min(cmp::min(possible_fuel.0, possible_fuel.1),possible_fuel.2);
        println!("Day 5 part ii: {}", 0.5 * best_fuel as f64);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
