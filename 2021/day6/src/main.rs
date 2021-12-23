use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Debug)]
struct Fish {
    spawns: Vec<u64>,
}

impl fmt::Display for Fish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.spawns)
    }
}

impl Fish {
    // The fish spawn a fish with a spawn interval of 8 when the spawn interval reaches 0. The
    // spawning fish also reset their spawn interval to 6.
    fn simulate(&mut self) {
        if self.spawns.len() < 2 {
            return;
        }

        let new_spawn = self.spawns[0];
        let len = self.spawns.len();

        for index in 0..len - 1 {
            self.spawns[index] = self.spawns[index + 1];
        }

        self.spawns[6] += new_spawn;
        self.spawns[8] = new_spawn;
    }
}

fn main() {
    let school = parse_input("assets/input.txt");

    println!(
        "school has {} fish after 18 days of simulation",
        simulate(&mut school.clone(), 18)
    );

    println!(
        "school has {} fish after 80 days of simulation",
        simulate(&mut school.clone(), 80)
    );

    println!(
        "school has {} fish after 256 days of simulation",
        simulate(&mut school.clone(), 256)
    );
}

// Parse the input file to generate the school of fish and their spawn intervals.
fn parse_input(path: &str) -> Fish {
    let file = match File::open(path) {
        Ok(handle) => handle,
        Err(err) => {
            panic!("invalid files {}: {}", path, err);
        }
    };

    let mut data = String::new();
    io::BufReader::new(file).read_line(&mut data).unwrap();

    let initial: Vec<u64> = data
        .split(",")
        .filter_map(|x| x.trim().parse::<u64>().ok())
        .collect();

    let mut fish = Fish { spawns: vec![0; 9] };

    for elem in initial {
        if elem > 8 {
            println!("element {} is out of bounds", elem);
            continue;
        }

        fish.spawns[elem as usize] += 1;
    }

    fish
}

// Simulate the fish school going through the spawning process for a given number of days.
fn simulate(school: &mut Fish, days: u64) -> u64 {
    for _ in 1..=days {
        school.simulate();
    }

    school.spawns.iter().sum()
}
