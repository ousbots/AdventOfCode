use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

const END: &str = "end";
const START: &str = "start";

// Parse the input file to generate the cave map.
fn parse_input(path: &str) -> HashMap<String, Vec<String>> {
    let file = match File::open(path) {
        Ok(handle) => handle,
        Err(err) => {
            panic!("invalid files {}: {}", path, err);
        }
    };

    let mut map = HashMap::<String, Vec<String>>::new();

    for line in io::BufReader::new(file).lines() {
        let tokens = line
            .unwrap()
            .split("-")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        if tokens.len() != 2 {
            panic!("unexpected tokens {:?}", tokens);
        }

        let entry = match map.entry(tokens[0].clone()) {
            Occupied(entry) => entry.into_mut(),
            Vacant(entry) => entry.insert(Vec::<String>::new()),
        };

        entry.push(tokens[1].clone());
    }

    map
}

fn find_paths(map: HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    let mut paths = Vec::<Vec<String>>::new();
    let mut visited = vec![START.to_string()];
    let mut queue = map[START].clone();

    while queue.len() > 0 {
        let current = queue.pop().unwrap();
        visited.push(current.clone());

        if current == END {
            paths.push(visited.clone());
        }
    }

    paths
}

fn main() {
    let map = parse_input("assets/input.txt");
}
