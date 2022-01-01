use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

// Parse the input file to generate the polymer and chain substitutions.
fn parse_input(path: &str) -> (String, HashMap<String, String>) {
    let file = match File::open(path) {
        Ok(handle) => handle,
        Err(err) => {
            panic!("invalid files {}: {}", path, err);
        }
    };

    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let polymer = lines[0].clone();

    if polymer.contains("->") {
        panic!("invalid polymer {}", polymer);
    }

    let subs: HashMap<String, String> = lines[2..]
        .into_iter()
        .map(|line| {
            let tokens: Vec<String> = line.split(" -> ").map(|x| x.to_string()).collect();
            if tokens.len() != 2 {
                panic!("invalid sub line {}", line);
            }

            if tokens[0].len() != 2 {
                panic!("invalid sub pair {}", tokens[0]);
            }

            let mut sub = tokens[0].clone();
            sub.insert(1, tokens[1].chars().last().unwrap());

            //let sub = tokens[1].chars().last().unwrap().to_string()
            //    + &tokens[0].chars().last().unwrap().to_string();

            (tokens[0].clone(), sub)
        })
        .collect();

    (polymer, subs)
}

// Updates the polymer pair count from a polymer chain.
fn update_pairs(polymer: String, pairs: &mut HashMap<String, u64>, count: u64) {
    for pair in polymer.chars().collect::<Vec<char>>().windows(2) {
        let entry = match pairs.entry(pair.iter().collect::<String>()) {
            Occupied(entry) => entry.into_mut(),
            Vacant(entry) => entry.insert(0),
        };

        *entry += count;
    }
}

// Update the polymer pair counts using the polymer substitutions.
fn substitute(pairs: &mut HashMap<String, u64>, subs: &HashMap<String, String>) {
    let mut temp = HashMap::<String, u64>::new();
    for (pair, count) in pairs.clone() {
        pairs.entry(pair.clone()).and_modify(|e| *e -= count);

        if pairs[&pair] == 0 {
            pairs.remove(&pair);
        }

        update_pairs(subs[&pair].chars().collect::<String>(), &mut temp, count);
    }

    for (key, value) in temp {
        let entry = match pairs.entry(key) {
            Occupied(entry) => entry.into_mut(),
            Vacant(entry) => entry.insert(0),
        };

        *entry += value;
    }
}

// Calculate the polymer number (most common element count - least common element count).
// FIXME: This is off by +1 in certain cases, but solves the problem for now.
fn polymer_number(pairs: &HashMap<String, u64>) -> u64 {
    let mut elements = HashMap::<char, u64>::new();

    for (pair, count) in pairs {
        for element in pair.chars() {
            let entry = match elements.entry(element) {
                Occupied(entry) => entry.into_mut(),
                Vacant(entry) => entry.insert(0),
            };

            *entry += count;
        }
    }

    let mut min = 'a';
    let mut max = 'a';

    for key in elements.keys() {
        let count = elements[key];

        if !elements.contains_key(&min) {
            min = *key;
        } else {
            if count < elements[&min] {
                min = *key;
            }
        }

        if !elements.contains_key(&max) {
            max = *key;
        } else {
            if count > elements[&max] {
                max = *key;
            }
        }
    }

    // Round up if there is a remainder.
    let mut count_max = elements[&max] / 2;
    if elements[&max] % 2 == 1 {
        count_max += 1;
    }

    let mut count_min = elements[&min] / 2;
    if elements[&min] % 2 == 1 {
        count_min += 1;
    }

    println!("least common element {} seen {} times", min, count_min);
    println!("most common element {} seen {} times", max, count_max);

    count_max - count_min
}

fn main() {
    let (polymer, subs) = parse_input("assets/input.txt");

    let mut pairs = HashMap::<String, u64>::new();
    update_pairs(polymer, &mut pairs, 1);

    for _ in 0..10 {
        substitute(&mut pairs, &subs);
    }

    println!(
        "polymer number after 10 iterations is {}",
        polymer_number(&pairs)
    );

    for _ in 0..30 {
        substitute(&mut pairs, &subs);
    }

    println!(
        "polymer number after 40 iterations is {}",
        polymer_number(&pairs)
    );
}
