/// Simulates the unloading of cargo boxes in stacks. There are two different cargo movers, the
/// cargo 9000 and the cargo 9001. The first one can only move one box at a time, while the second
/// one is capable of moving multiple boxes from the same stack at once.
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Process the input file and simulate the cargo movement for both cargo movers.
fn simulate(path: String) -> Result<(), Box<dyn Error>> {
    let mut input = BufReader::new(File::open(path)?);
    let mut cargo_9000 = parse_cargo(&mut input)?;

    let mut cargo_9001 = vec![];
    for stack in cargo_9000.iter() {
        let mut temp = vec![];

        for unit in stack {
            temp.push(*unit);
        }
        cargo_9001.push(temp);
    }

    let (top_9000, top_9001) = simulate_cargo(input, &mut cargo_9000, &mut cargo_9001)?;

    println!(
        "the top cargo boxes for a cargo 9000 are {}",
        top_9000.iter().collect::<String>()
    );
    println!(
        "the top cargo boxes for a cargo 9001 are {}",
        top_9001.iter().collect::<String>()
    );

    Ok(())
}

// Parse the first part of the input file to build the cargo array.
// Note: The layout of the cargo input is:
//       [G]
//       [E]     [F]
//   [A] [B] [C] [D]
//    1   2   3   4
//
//   Where each cargo stack is represented by 4 characters, a '[', then the cargo identifier,
//   then a ']', and finally a space. The last row of the cargo input file is the list of stack
//   numbers, but this can be determined by dividing the index by 4 as well.
fn parse_cargo(input: &mut BufReader<File>) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let mut cargo = vec![];

    for raw in input.lines() {
        let line = raw?;
        let tokens: Vec<char> = line.chars().collect();

        // A blank line in the input separates the cargo from the moves.
        if line.trim().len() == 0 {
            break;
        }

        if cargo.len() == 0 {
            for _ in 0..(tokens.len() / 4) + 1 {
                cargo.push(vec![]);
            }
        }

        for i in (0..tokens.len()).step_by(4) {
            if tokens[i] == '[' && tokens[i + 2] == ']' {
                cargo[i / 4].push(tokens[i + 1]);
            }
        }
    }

    for i in 0..cargo.len() {
        cargo[i] = cargo[i].to_owned().into_iter().rev().collect();
    }

    Ok(cargo)
}

// Simulate the cargo moves to determine the top boxes on each final stack. The cargo 9001 can move
// multiple boxes at a time and the cargo 9000 can only move one box at a time.
fn simulate_cargo(
    input: BufReader<File>,
    cargo_9000: &mut Vec<Vec<char>>,
    cargo_9001: &mut Vec<Vec<char>>,
) -> Result<(Vec<char>, Vec<char>), Box<dyn Error>> {
    for raw in input.lines() {
        let line = raw?;
        let tokens: Vec<&str> = line.split(' ').collect();

        // Ensure that the line is a move instruction.
        if tokens[0] != "move" || tokens[2] != "from" || tokens[4] != "to" {
            panic!("invalid instruction line {}", line);
        }

        let count: usize = tokens[1].parse()?;
        let from: usize = tokens[3].parse()?;
        let to: usize = tokens[5].parse()?;

        // Move the boxes one at a time, which is copying in reverse order.
        let len = cargo_9000[from - 1].len();
        let mut transit_9000: Vec<char> = cargo_9000[from - 1]
            .split_off(len - count)
            .iter()
            .cloned()
            .rev()
            .collect();
        cargo_9000[to - 1].append(&mut transit_9000);

        // Move the boxes all at once.
        let len = cargo_9001[from - 1].len();
        let mut transit_9001 = cargo_9001[from - 1].split_off(len - count);
        cargo_9001[to - 1].append(&mut transit_9001);
    }

    let tops_9000: Vec<char> = cargo_9000.iter_mut().filter_map(|x| x.pop()).collect();
    let tops_9001: Vec<char> = cargo_9001.iter_mut().filter_map(|x| x.pop()).collect();

    Ok((tops_9000, tops_9001))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("expected the input file path, not {:?}", &args[1..]);
    }

    simulate(args[1].to_owned()).expect("failed to simulate cargo");
}
