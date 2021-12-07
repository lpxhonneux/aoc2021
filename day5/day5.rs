use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./day5.txt") {
        let mut line_coords = lines.map(|x| x.unwrap())
                               .map(|x| x.split(&[' ','-','>',','][..])
                                         .filter(|y| !y.is_empty())
                                         .map(|y| y.parse::<u32>().unwrap())
                                         .collect::<Vec<u32>>()
                               );
        println!("Day 5 part i: {:?}", line_coords.nth(0));
        println!("Day 5 part ii: {}", 0);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
