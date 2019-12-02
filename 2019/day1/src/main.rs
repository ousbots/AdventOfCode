use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn required_fuel(mass: u64) -> u64 {
    let mut temp_mass: f64 = mass as f64;
    let mut fuel: u64 = 0;

    while temp_mass >= 6.0 {
        temp_mass = (temp_mass / 3.0).floor() - 2.0;
        fuel += temp_mass as u64;
    }

    return fuel;
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
        total += required_fuel(line?.parse::<u64>().unwrap());
    }

    println!("total fuel: {}", total);

    Ok(())
}
