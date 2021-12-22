use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let path = "assets/input.txt";

    let file = match File::open(path) {
        Ok(handle) => handle,
        Err(err) => {
            println!("invalid files {}: {}", path, err);
            return;
        }
    };

    let commands: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| match x {
            Ok(cmd) => cmd,
            Err(err) => {
                println!("invalid command parsed: {}", err);
                "".to_string()
            }
        })
        .collect();

    if commands.len() == 0 {
        println!("need more than {} commands to compute", commands.len());
        return;
    }

    let (horizontal_bad, depth_bad) = calculate_bad_position(&commands);
    println!(
        "final position from the 'bad' algorithm will be {} horizontal and {} deep ({})",
        horizontal_bad,
        depth_bad,
        horizontal_bad * depth_bad
    );

    let (horizontal, depth) = calculate_position(&commands);
    println!(
        "final position from the 'good' algorithm will be {} horizontal and {} deep ({})",
        horizontal,
        depth,
        horizontal * depth
    );
}

// Calculates the final horizontal and depth positions from the given commands using the "bad"
// algorithm.
fn calculate_bad_position(commands: &Vec<String>) -> (i32, i32) {
    let mut x_pos = 0;
    let mut y_pos = 0;

    for command in commands {
        let tokens: Vec<&str> = command.split(" ").collect();
        if tokens.len() != 2 {
            println!("found incorrect number of tokens {:?}, skipping", tokens);
            continue;
        }

        let magnitude = match tokens[1].parse::<i32>() {
            Ok(num) => num,
            Err(err) => {
                println!("invalid magnitude: {}", err);
                0
            }
        };

        match tokens[0] {
            "forward" => x_pos += magnitude,
            "up" => y_pos -= magnitude,
            "down" => y_pos += magnitude,
            _ => println!("skipping unknown token {}", tokens[0]),
        }
    }

    (x_pos, y_pos)
}

// Calculates the final horizontal and depth positions from the given commands using the "good"
// algorithm.
fn calculate_position(commands: &Vec<String>) -> (i32, i32) {
    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut aim = 0;

    for command in commands {
        let tokens: Vec<&str> = command.split(" ").collect();
        if tokens.len() != 2 {
            println!("found incorrect number of tokens {:?}, skipping", tokens);
            continue;
        }

        let magnitude = match tokens[1].parse::<i32>() {
            Ok(num) => num,
            Err(err) => {
                println!("invalid magnitude: {}", err);
                0
            }
        };

        match tokens[0] {
            "forward" => {
                x_pos += magnitude;
                y_pos += aim * magnitude;
            }
            "up" => aim -= magnitude,
            "down" => aim += magnitude,
            _ => println!("skipping unknown token {}", tokens[0]),
        }
    }

    (x_pos, y_pos)
}
