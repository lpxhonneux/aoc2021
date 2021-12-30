use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashMap;

fn draw_line(canvas: &mut HashMap<(i32,i32),i32>, coords: (i32,i32,i32,i32)) {
    let direction = ((coords.2-coords.0).signum(), (coords.3-coords.1).signum());
    let mut cur = (coords.0,coords.1);
    let end = (coords.2, coords.3);
    let first_coord_counter = canvas.entry(cur).or_insert(0);
    *first_coord_counter += 1;
    loop {
        if cur == end { break; }
        cur.0 += direction.0;
        cur.1 += direction.1;
        let counter = canvas.entry(cur).or_insert(0);
        *counter += 1;
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./day5.txt") {
        let line_coords = lines.map(|x| x.unwrap())
                               .map(|x| x.split(&[' ','-','>',','][..])
                                         .filter(|y| !y.is_empty())
                                         .map(|y| y.parse::<i32>().unwrap())
                                         .collect::<Vec<i32>>()
                               );
        let (straight, diag):(Vec<(i32,i32,i32,i32)>,Vec<(i32,i32,i32,i32)>) = line_coords.map(|v| (v[0],v[1],v[2],v[3]))
                                          .partition(|(x0,y0,x1,y1)| x0 == x1 || y0 == y1 );
        let mut canvas = HashMap::new();
        for line in straight { draw_line(&mut canvas, line) }
        println!("Day 5 part i: {:?}", canvas.values().filter(|x| **x > 1).count());
        for line in diag { draw_line(&mut canvas, line) }
        println!("Day 5 part ii: {}", canvas.values().filter(|x| ** x > 1).count());
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
