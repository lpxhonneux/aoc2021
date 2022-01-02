use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashMap;
use std::cmp::min;
use std::cmp::max;

pub fn apply_rules(polymer: &Vec<char>, rules: &HashMap<(char,char),char>) -> Vec<char> {
    polymer.windows(2)
        .fold(
            Vec::new(),
            |mut acc, sli | {
                if acc.is_empty() {
                    acc.push(sli[0]);
                }
                if let Some(c) = rules.get(&(sli[0],sli[1])) {
                    acc.push(*c);
                    acc.push(sli[1]);
                }
                acc
            }
        )
}

pub fn count_most_and_least_common(polymer: &Vec<char>) -> (u64,u64) {
    let mut counts = HashMap::new();
    for c in polymer {
        let counter = counts.entry(c).or_insert(0);
        *counter += 1;
    }

    counts.drain()
        .fold(
            (u64::MAX,0),
            |(low, high), (_k,v)| {
                (min(low,v), max(high, v))
            }
        )
}

pub fn apply_rules_2(polymer: &HashMap<(char,char),u64>,rules: &HashMap<(char,char),char>) -> HashMap<(char,char),u64> {
    polymer.iter()
        .fold(
            HashMap::new(),
            |mut acc, (k,v)| {
                if let Some(c) = rules.get(&k) {
                    let mut counter = acc.entry((k.0,*c)).or_insert(0);
                    *counter += v;
                    counter = acc.entry((*c,k.1)).or_insert(0);
                    *counter += v;
                }
                acc
            }
        )
}

pub fn convert_polymer_repr(polymer: &Vec<char>) -> (char,char,HashMap<(char,char),u64>) {
    let first = polymer[0];
    let last = *polymer.last().unwrap();
    let map_repr = polymer.windows(2)
        .fold(
            HashMap::new(),
            |mut acc, sli| {
                let counter = acc.entry((sli[0],sli[1])).or_insert(0);
                *counter += 1;
                acc
            }
        );
    (first,last,map_repr)
}

pub fn count_most_and_least_common_2(first:char, last:char, polymer: &mut HashMap<(char,char), u64>) -> (u64,u64) {
    let mut counts = HashMap::new();
    counts.insert(first,1);
    counts.insert(last,1);
    for (k,v) in polymer.drain() {
        let mut counter = counts.entry(k.0).or_insert(0);
        *counter += v;
        counter = counts.entry(k.1).or_insert(0);
        *counter += v;
    }

    counts.drain()
        .fold(
            (u64::MAX,0),
            |(low, high), (_k,v)| {
                (min(low,v), max(high, v))
            }
        )
}

fn main() {
    // File hosts must exist in current path before this produces output
    println!("DAY 14");
    if let Ok(mut lines) = read_lines("./day14.txt") {
        let mut polymer = lines.nth(0).unwrap().unwrap().chars().collect();
        let rules : HashMap<(char,char),char> = lines.skip(1)
            .map(|x| x.unwrap())
            .map(|x| {
                let mut rule = x.split(" -> ");
                let mut rule_head = rule.nth(0).unwrap().chars();
                let rule_tail = rule.nth(0).unwrap().chars().nth(0).unwrap();
                (
                    (
                        rule_head.nth(0).unwrap(),
                        rule_head.nth(0).unwrap(),
                    ),
                    rule_tail
                )
            })
            .collect();
        let (first, last, mut polymer_new_repr) = convert_polymer_repr(&polymer);
        for _ in 0..10 {
            polymer = apply_rules(&polymer, &rules);
        }
        let (lcc_10, mcc_10) = count_most_and_least_common(&polymer);
        println!("PART I: {:?}", mcc_10-lcc_10);
        for _ in 0..40 {
            polymer_new_repr = apply_rules_2(&polymer_new_repr, &rules);
        }
        let (lcc_40, mcc_40) = count_most_and_least_common_2(first, last, &mut polymer_new_repr);
        println!("PART II: {:?}", (mcc_40-lcc_40)/2);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
