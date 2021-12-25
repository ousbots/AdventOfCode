use std::fs::File;
use std::io::{self, BufRead};

// Parse the input file to generate the map of dumbo energy levels.
fn parse_input(path: &str) -> Vec<Vec<i32>> {
    let file = match File::open(path) {
        Ok(handle) => handle,
        Err(err) => {
            panic!("invalid files {}: {}", path, err);
        }
    };

    let dumbos: Vec<Vec<i32>> = io::BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .filter_map(|num| num.to_string().parse::<i32>().ok())
                .collect()
        })
        .collect();

    dumbos
}

// Simulate a number of time steps for all dumbos and caluclate the total number of flashes.
fn flashes_in_steps(dumbos: &mut Vec<Vec<i32>>, steps: usize) -> i32 {
    let mut flashes = 0;

    for _ in 0..steps {
        flashes += simulate_flashes(dumbos);
    }

    flashes
}

// Calculate how many steps it will take for the dumbo flashes to synchronize.
fn steps_to_sync(dumbos: &mut Vec<Vec<i32>>) -> i32 {
    let mut steps = 0;
    let total_dumbos = dumbos.len() * dumbos[0].len();

    loop {
        steps += 1;
        if simulate_flashes(dumbos) == i32::try_from(total_dumbos).unwrap() {
            return steps;
        }
    }
}

// Simulate a single time step for all dumbos and calculate the number of flashes that happened.
fn simulate_flashes(dumbos: &mut Vec<Vec<i32>>) -> i32 {
    let mut flashes = 0;
    for x in 0..dumbos.len() {
        for y in 0..dumbos[x].len() {
            dumbos[x][y] += 1;

            if dumbos[x][y] == 10 {
                flashes += flash_dumbos(dumbos, x, y);
            }
        }
    }

    for x in 0..dumbos.len() {
        for y in 0..dumbos[x].len() {
            if dumbos[x][y] > 9 {
                dumbos[x][y] = 0;
            }
        }
    }

    flashes
}

// Recursively simulates the flashing dumbo affecting it's adjacent neighbors and calculates the
// total number of flashes.
fn flash_dumbos(dumbos: &mut Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let mut flashes = 1;

    let x_max = dumbos.len();
    let y_max = dumbos[0].len();

    if x > 0 {
        dumbos[x - 1][y] += 1;
        if dumbos[x - 1][y] == 10 {
            flashes += flash_dumbos(dumbos, x - 1, y);
        }

        if y > 0 {
            dumbos[x - 1][y - 1] += 1;
            if dumbos[x - 1][y - 1] == 10 {
                flashes += flash_dumbos(dumbos, x - 1, y - 1);
            }
        }

        if y < y_max - 1 {
            dumbos[x - 1][y + 1] += 1;
            if dumbos[x - 1][y + 1] == 10 {
                flashes += flash_dumbos(dumbos, x - 1, y + 1);
            }
        }
    }

    if x < x_max - 1 {
        dumbos[x + 1][y] += 1;
        if dumbos[x + 1][y] == 10 {
            flashes += flash_dumbos(dumbos, x + 1, y);
        }

        if y < y_max - 1 {
            dumbos[x + 1][y + 1] += 1;
            if dumbos[x + 1][y + 1] == 10 {
                flashes += flash_dumbos(dumbos, x + 1, y + 1)
            }
        }

        if y > 0 {
            dumbos[x + 1][y - 1] += 1;
            if dumbos[x + 1][y - 1] == 10 {
                flashes += flash_dumbos(dumbos, x + 1, y - 1);
            }
        }
    }

    if y > 0 {
        dumbos[x][y - 1] += 1;
        if dumbos[x][y - 1] == 10 {
            flashes += flash_dumbos(dumbos, x, y - 1);
        }
    }

    if y < y_max - 1 {
        dumbos[x][y + 1] += 1;
        if dumbos[x][y + 1] == 10 {
            flashes += flash_dumbos(dumbos, x, y + 1);
        }
    }

    flashes
}

fn main() {
    let mut dumbos = parse_input("assets/input.txt");

    let flashes = flashes_in_steps(&mut dumbos.clone(), 100);
    println!("dumbos flashed {} times in 100 steps", flashes);

    println!(
        "took {} steps to synchronize the dumbo flashes",
        steps_to_sync(&mut dumbos)
    );
}
