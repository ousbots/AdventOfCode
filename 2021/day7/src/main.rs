use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let positions = parse_input("assets/input.txt");

    let (min_position, min_moves) = calculate_minimal_position(positions.clone());
    println!(
        "minimal position is {} and requires {} moves",
        min_position, min_moves
    );

    let (real_min_position, real_min_moves) = calculate_real_minimal_position(positions);
    println!(
        "real minimal position is {} and requires {} moves",
        real_min_position, real_min_moves
    );
}

// Parse the input file to generate the list of horizontal positions.
fn parse_input(path: &str) -> HashMap<i64, i64> {
    let file = match File::open(path) {
        Ok(handle) => handle,
        Err(err) => {
            panic!("invalid files {}: {}", path, err);
        }
    };

    let mut data = String::new();
    io::BufReader::new(file).read_line(&mut data).unwrap();

    let initial: Vec<i64> = data
        .split(",")
        .filter_map(|x| x.trim().parse::<i64>().ok())
        .collect();

    let mut positions = HashMap::<i64, i64>::new();

    for elem in initial {
        let entry = match positions.entry(elem) {
            Vacant(entry) => entry.insert(0),
            Occupied(entry) => entry.into_mut(),
        };

        *entry += 1;
    }

    positions
}

// Calculates the postion that requires the minimal amount of moves from all other positions.
fn calculate_minimal_position(positions: HashMap<i64, i64>) -> (i64, i64) {
    let mut min_moves = 0;
    let mut min_position = 0;

    let max_value = *positions.keys().max_by(|x, y| x.cmp(y)).unwrap();

    for reference in 0..max_value {
        let mut moves = 0;

        for (key, val) in &positions {
            moves += (key - reference).abs() * val;
        }

        if min_moves == 0 || moves < min_moves {
            min_position = reference;
            min_moves = moves;
        }
    }

    (min_position, min_moves)
}

// Calculates the postion that requires the minimal amount of moves from all other positions using
// the "real" algorithm that makes each move cost one more than the last.
fn calculate_real_minimal_position(positions: HashMap<i64, i64>) -> (i64, i64) {
    let mut min_moves = 0;
    let mut min_position = 0;

    let max_value = *positions.keys().max_by(|x, y| x.cmp(y)).unwrap();

    for reference in 0..max_value {
        let mut moves = 0;

        for (key, val) in &positions {
            let n = (key - reference).abs();
            moves += (n * (n + 1) / 2) * val;
        }

        if min_moves == 0 || moves < min_moves {
            min_position = reference;
            min_moves = moves;
        }
    }

    (min_position, min_moves)
}
