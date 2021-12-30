use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn or(a: [bool;7], b: [bool;7]) -> [bool;7] {
    let mut result = [false;7];
    for i in 0..7 {
        result[i] = a[i] | b[i];
    }
    result
}

fn and(a: [bool;7], b: [bool;7]) -> [bool;7] {
    let mut result = [false;7];
    for i in 0..7 {
        result[i] = a[i] & b[i];
    }
    result
}

fn xor(a: [bool;7], b: [bool;7]) -> [bool;7] {
    let mut result = [false;7];
    for i in 0..7 {
        result[i] = a[i] ^ b[i];
    }
    result
}

fn neg(a: [bool;7]) -> [bool;7] {
    let mut result = [false;7];
    for i in 0..7 {
        result[i] = !a[i];
    }
    result
}

fn minus(a: [bool;7], b: [bool;7]) -> [bool;7] {
    xor(a, and(a,b))
}

fn bool_to_char(a: [bool;7]) -> char {
    let answers = ['a','b','c','d','e','f','g'];
    for ith in 0..7 {
        if a[ith] { return answers[ith];}
    }
    'a'
}

fn decode(input: Vec<[bool;7]>) -> [char; 7] {
    let five_inter : [bool;7] = input.iter().cloned()
                          .filter(|x| x.iter().filter(|y| **y).count() == 5)
                          .reduce(|acc, x| and(acc,x) ).unwrap();
    let six_inter = input.iter().cloned()
                          .filter(|x| x.iter().filter(|y| **y).count() == 6)
                          .reduce(|acc, x| and(acc,x) ).unwrap();
    let mid_mid = minus(five_inter, six_inter);
    let top_mid = input.iter().cloned()
                       .filter(|x| x.iter().filter(|y| **y).count() == 3 || x.iter().filter(|y| **y).count() == 2)
                       .reduce(|acc, x| xor(acc,x)).unwrap();
    let bot_mid = minus(minus(five_inter, top_mid), mid_mid);
    let one = input.iter().cloned()
                          .filter(|x| x.iter().filter(|y| **y).count() == 2)
                          .reduce(|acc, x| and(acc,x) ).unwrap();
    let four = input.iter().cloned()
                          .filter(|x| x.iter().filter(|y| **y).count() == 4)
                          .reduce(|acc, x| and(acc,x) ).unwrap();
    let seven = input.iter().cloned()
                          .filter(|x| x.iter().filter(|y| **y).count() == 3)
                          .reduce(|acc, x| and(acc,x) ).unwrap();
    let eight = input.iter().cloned()
                          .filter(|x| x.iter().filter(|y| **y).count() == 7)
                          .reduce(|acc, x| and(acc,x) ).unwrap();
    let bottom_left = minus(eight, or(four,five_inter));
    let top_left = minus(eight, or(seven,or(five_inter,bottom_left)));
    let top_right = minus(eight, or(six_inter, or(bottom_left,mid_mid)));
    let bottom_right = minus(one, top_right);

    let mut result = [ 'a'; 7 ];
    result[0] = bool_to_char(top_mid);
    result[1] = bool_to_char(top_left);
    result[2] = bool_to_char(top_right);
    result[3] = bool_to_char(mid_mid);
    result[4] = bool_to_char(bottom_left);
    result[5] = bool_to_char(bottom_right);
    result[6] = bool_to_char(bot_mid);
    return result;
}

fn encode_char(state: [bool;7], c: char) -> [bool;7] {
    let mut result = state;
    match c {
        'a' => { result[0] = true; },
        'b' => { result[1] = true; },
        'c' => { result[2] = true; },
        'd' => { result[3] = true; },
        'e' => { result[4] = true; },
        'f' => { result[5] = true; },
        'g' => { result[6] = true; },
        _   => ()
    }
    result
}

fn encode_vec_char(s: Vec<char>) -> [bool;7] {
    s.into_iter().fold([false;7], |acc, c| encode_char(acc,c))
}


fn create_numbers(key: [char;7]) -> [[bool;7];10] {
    let zero = encode_vec_char(vec![key[0],key[1],key[2],key[4],key[5],key[6]]);
    let one = encode_vec_char(vec![key[2],key[5]]);
    let two = encode_vec_char(vec![key[0],key[2],key[3],key[4],key[6]]);
    let three = encode_vec_char(vec![key[0],key[2],key[3],key[5],key[6]]);
    let four = encode_vec_char(vec![key[1],key[2],key[3],key[5]]);
    let five = encode_vec_char(vec![key[0],key[1],key[3],key[5],key[6]]);
    let six = encode_vec_char(vec![key[0],key[1],key[3],key[4],key[5],key[6]]);
    let seven = encode_vec_char(vec![key[0],key[2],key[5]]);
    let eight = encode_vec_char(vec![key[0],key[1],key[2],key[3],key[4],key[5],key[6]]);
    let nine = encode_vec_char(vec![key[0],key[1],key[2],key[3],key[5],key[6]]);
    return [zero, one, two, three, four, five, six, seven, eight, nine]
}

fn transcribe(key: [char;7], enc_num : [bool;7]) -> usize {
    let numbers = create_numbers(key);
    for (i,n) in numbers.iter().enumerate() {
        if *n == enc_num {
            return i;
        }
    }
    0
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./day8.txt") {
        let total = lines.map(|x| x.unwrap())
                          .map(|x| x.split(" | ").nth(1).unwrap()
                                    .split_whitespace()
                                    .map(|y| y.chars()
                                              .count()
                                    ).collect::<Vec<usize>>()
                          )
                          .map(
                              |v| {
                                  let count = v.iter().fold([0;10],
                                        |mut acc, x| {
                                            acc[*x] += 1;
                                            acc
                                        }
                                   );
                                   count[2] + count[3] + count[4] + count[7]
                                }
                          ).sum::<usize>();
        println!("Day 8 part i: {}", total);
    }
    if let Ok(lines) = read_lines("./day8.txt") {
        let key = lines.map(|x| x.unwrap())
                          .map(|x| {
                              let mut line = x.split(" | ");
                              (line.nth(0).unwrap()
                                    .split_whitespace()
                                    .map(|y| y.chars()
                                              .fold([false;7], |acc, x| {
                                                  encode_char(acc, x)
                                              })
                                    ).collect::<Vec<[bool;7]>>(),
                               line.nth(0).unwrap()
                                    .split_whitespace()
                                    .map(|y| y.chars()
                                              .fold([false;7], |acc, x| {
                                                  encode_char(acc, x)
                                              })
                                    ).collect::<Vec<[bool;7]>>()
                               )
                           }
                          ).map( |input| (decode(input.0),input.1) );
        let answers = key.map(|(k, out)| {
            out.into_iter().map(|x| transcribe(k, x))
                                            .fold(0, |acc, y| acc * 10 + y)
                }
                              );
        println!("Day 8 part ii: {}", answers.sum::<usize>());
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
