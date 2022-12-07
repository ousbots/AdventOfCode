/// Analyzes a signal input to find the start packet and the message packet. The start packet is
/// the first four token frame that contains unique values. The message packet is the first
/// fourteen token fram that contains unique values.
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Process the signal input and find the start and message packets.
fn run(path: String) -> Result<(), Box<dyn Error>> {
    for raw in BufReader::new(File::open(path)?).lines() {
        let line = raw?;
        let tokens: Vec<char> = line.chars().collect();

        let start_packet = find_unique_packet(&tokens, 4)?;
        let message_packet = find_unique_packet(&tokens, 14)?;

        println!(
            "the start packet is {} and the signal starts at {}",
            tokens[start_packet..start_packet + 4]
                .iter()
                .collect::<String>(),
            start_packet + 4,
        );

        println!(
            "the message packet is {} and the message starts at {}",
            tokens[message_packet..message_packet + 14]
                .iter()
                .collect::<String>(),
            message_packet + 14,
        );
    }

    Ok(())
}

// Find the first unique packet of the given length.
fn find_unique_packet(tokens: &Vec<char>, length: usize) -> Result<usize, Box<dyn Error>> {
    let mut frame: HashMap<char, i32> = HashMap::new();
    let mut start = 0;

    'toploop: for index in 0..tokens.len() {
        if index >= length {
            let entry = frame.entry(tokens[start]).or_insert(0);
            *entry -= 1;
            if *entry <= 0 {
                frame.remove(&tokens[start]);
            }
            start += 1;
        }

        let entry = frame.entry(tokens[index]).or_insert(0);
        *entry += 1;

        if index >= length {
            for count in frame.values() {
                if *count != 1 {
                    continue 'toploop;
                }
            }

            break 'toploop;
        }
    }

    Ok(start)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!(
            "expected only the path to the input file, not {:?}",
            &args[1..]
        );
    }

    run(args[1].to_owned()).expect("failed to process the signal");
}
