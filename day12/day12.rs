use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub enum Node {
    START,
    END,
    SMALL(u32),
    LARGE(u32),
}

type CavePath = Vec<Node>;

#[derive(Clone,Debug)]
pub struct CaveMap {
    edges: HashMap<Node, Vec<Node>>
}

impl CaveMap {
    pub fn new() -> Self {
        CaveMap {
            edges : HashMap::new(),
        }
    }

    pub fn all_paths(&self) -> Vec<CavePath> {
        let mut paths = Vec::new();
        let mut queue = VecDeque::<(Vec::<Node>,HashSet::<Node>)>::new();
        for node in self.edges.get(&Node::START).unwrap() {
            let mut seen = HashSet::<Node>::new();
            match node {
                Node::START => (),
                Node::END   => {  paths.push(vec![Node::START,Node::END]); },
                Node::SMALL(_) => { seen.insert(*node);
                                    queue.push_back((vec![Node::START,*node], seen.clone())); },
                Node::LARGE(_) => { queue.push_back((vec![Node::START,*node], seen.clone()));},
            }
        }
        while let Some((path_prefix, seen)) = queue.pop_front() {
            let last = path_prefix.last().unwrap();
            for node in self.edges.get(last).unwrap() {
                if !seen.contains(node) {
                    match node {
                        Node::START => (),
                        Node::END   => { let mut new_path = path_prefix.clone(); new_path.push(Node::END); paths.push(new_path); },
                        Node::SMALL(_) => { let mut new_seen = seen.clone();
                                            new_seen.insert(*node);
                                            let mut new_path = path_prefix.clone();
                                            new_path.push(*node);
                                            queue.push_back((new_path, new_seen)); },
                        Node::LARGE(_) => { let mut new_path = path_prefix.clone();
                                            new_path.push(*node);
                                            queue.push_back((new_path, seen.clone()));},
                    }
                }
            }
        }
        return paths;
    }

    pub fn all_paths_newrules(&self) -> Vec<CavePath> {
        let mut paths = Vec::new();
        let mut queue = VecDeque::<(Vec::<Node>,HashSet::<Node>,bool)>::new();
        for node in self.edges.get(&Node::START).unwrap() {
            let mut seen = HashSet::<Node>::new();
            match node {
                Node::START => (),
                Node::END   => {  paths.push(vec![Node::START,Node::END]); },
                Node::SMALL(_) => { seen.insert(*node);
                                    queue.push_back((vec![Node::START,*node], seen.clone(),false)); },
                Node::LARGE(_) => { queue.push_back((vec![Node::START,*node], seen.clone(),false));},
            }
        }
        while let Some((path_prefix, seen, visit_twice)) = queue.pop_front() {
            let last = path_prefix.last().unwrap();
            for node in self.edges.get(last).unwrap() {
                if (!seen.contains(node)) || (!visit_twice) {
                    let visit = if !visit_twice { seen.contains(node) } else { true };
                    match node {
                        Node::START => (),
                        Node::END   => { let mut new_path = path_prefix.clone(); new_path.push(Node::END); paths.push(new_path); },
                        Node::SMALL(_) => { let mut new_seen = seen.clone();
                                            new_seen.insert(*node);
                                            let mut new_path = path_prefix.clone();
                                            new_path.push(*node);
                                            queue.push_back((new_path, new_seen,visit)); },
                        Node::LARGE(_) => { let mut new_path = path_prefix.clone();
                                            new_path.push(*node);
                                            queue.push_back((new_path, seen.clone(),visit));},
                    }
                }
            }
        }
        return paths;
    }
}

pub fn hash_node_name(s: &str) -> Node {
    match s {
        "start" => Node::START,
        "end"   => Node::END,
        _       => {
            let small = *s == s.to_lowercase();
            let value = s.to_lowercase().chars()
                         .fold(0, |acc, c|
                               acc*26 + (c as u32) - ('a' as u32)
            );
            if small { Node::SMALL(value) } else { Node::LARGE(value) }
        }
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./day12.txt") {
        let graph = lines.map(|x| x.unwrap())
            .map(|x| {
                let mut s = x.split('-');
                (hash_node_name(s.nth(0).unwrap()),hash_node_name(s.nth(0).unwrap()))
            })
            .fold(
                CaveMap::new(),
                |mut map, (i,j)| {
                    if let Some(v) = map.edges.get_mut(&i) {
                        v.push(j);
                    } else {
                        map.edges.insert(i, vec![j]);
                    }
                    if let Some(v) = map.edges.get_mut(&j) {
                        v.push(i);
                    } else {
                        map.edges.insert(j, vec![i]);
                    }
                    map
                }
            );
        println!("Day 12 part i: {}", graph.all_paths().len());
        println!("Day 12 part ii: {:?}", graph.all_paths_newrules().len());
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
