use std::fs;
use std::io;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let mut orbits: HashMap<&str, &str> = HashMap::new();

    // Parse the input file to a memory vector
    let input: String = fs::read_to_string("input")?;

    for line in input.lines() {
        let tokens = line.split(")").collect::<Vec<&str>>();

        if tokens.len() != 2 {
            panic!("bad orbit format")
        }

        let inner = tokens[0];
        let outer = tokens[1];
        orbits.insert(outer, inner);
    }

    // Calculate the total number of orbits
    let mut total_orbits: u64 = 0;

    for key in orbits.keys() {
        let mut pos = key;

        loop {
            if let Some(planet) = orbits.get(pos) {
                pos = planet;
                total_orbits += 1;
            }

            else {
                break;
            }
        }
    }

    println!("{} total orbits", total_orbits);

    let mut first_path: Vec<&str> = Vec::new();
    let mut pos = "YOU";
    loop {
        if let Some(planet) = orbits.get(pos) {
            pos = planet;
            first_path.push(planet);
        }

        else {
            break;
        }
    }

    let mut second_path: Vec<&str> = Vec::new();
    let mut pos = "SAN";
    loop {
        if let Some(planet) = orbits.get(pos) {
            pos = planet;
            second_path.push(planet);
        }

        else {
            break;
        }
    }

    let mut path_map: HashMap<&str, u64> = HashMap::new();

    for elem in &first_path {
        let mut count: u64 = 1;

        if let Some(num) = path_map.get(elem) {
            count += num;
        }

        path_map.insert(elem, count);
    }

    for elem in &second_path {
        let mut count: u64 = 1;

        if let Some(num) = path_map.get(elem) {
            count += num;
        }

        path_map.insert(elem, count);
    }

    let mut shortest_path: u64 = 0;
    for (_, seen) in path_map.iter() {
        if *seen == 1 {
            shortest_path += 1;
        }
    }

    println!("shortest path: {}", shortest_path);

    Ok(())
}
