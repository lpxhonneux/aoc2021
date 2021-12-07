use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./day1.txt") {
        let num_inc_readings = lines.map(|x| x.unwrap().parse::<i32>().unwrap())
                                    .collect::<Vec<i32>>()
                                    .windows(2)
                                    .fold(0, |acc, x| if x[1] > x[0] { acc + 1 } else { acc });
        println!("Day 1: {}", num_inc_readings)
        // Consumes the iterator, returns an (Optional) String
    }
    if let Ok(lines) = read_lines("./day1.txt") {
        let num_inc_readings_ii = lines.map(|x| x.unwrap().parse::<i32>().unwrap())
                                    .collect::<Vec<i32>>()
                                    .windows(4)
                                    .fold(0, |acc, x| if x[3] > x[0] { acc + 1 } else { acc });
        println!("Day 2: {}", num_inc_readings_ii)
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
