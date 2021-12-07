use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(mut lines) = read_lines("./day6.txt") {
        let mut time = 0;
        let mut hist_of_fish = lines.nth(0).unwrap().unwrap().split(',')
                                 .map(|x| x.parse::<usize>().unwrap())
                                 .fold([0;9], |mut acc, x| {acc[x] +=1; acc} );
        let mut next_hist = [0;9];
        loop {
            next_hist[0] = hist_of_fish[1];
            next_hist[1] = hist_of_fish[2];
            next_hist[2] = hist_of_fish[3];
            next_hist[3] = hist_of_fish[4];
            next_hist[4] = hist_of_fish[5];
            next_hist[5] = hist_of_fish[6];
            next_hist[6] = hist_of_fish[7]+hist_of_fish[0];
            next_hist[7] = hist_of_fish[8];
            next_hist[8] = hist_of_fish[0];

            for i in 0..9 { hist_of_fish[i] = next_hist[i]; }

            time += 1;

            if time == 80 { break; }
        }
        println!("Day 6 part i: {}", hist_of_fish.iter().sum::<usize>());
        loop {
            next_hist[0] = hist_of_fish[1];
            next_hist[1] = hist_of_fish[2];
            next_hist[2] = hist_of_fish[3];
            next_hist[3] = hist_of_fish[4];
            next_hist[4] = hist_of_fish[5];
            next_hist[5] = hist_of_fish[6];
            next_hist[6] = hist_of_fish[7]+hist_of_fish[0];
            next_hist[7] = hist_of_fish[8];
            next_hist[8] = hist_of_fish[0];

            for i in 0..9 { hist_of_fish[i] = next_hist[i]; }

            time += 1;

            if time == 256 { break; }
        }
        println!("Day 6 part ii: {}", hist_of_fish.iter().sum::<usize>());
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
