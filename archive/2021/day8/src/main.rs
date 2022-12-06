use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Signal {
    input: Vec<String>,
    output: Vec<String>,
}

fn main() {
    let signals = parse_input("assets/input.txt");

    let unique_output_digits = unique_output_digits(&signals);
    println!("found {} unique output digits", unique_output_digits);

    let mut output_sum = 0;
    for signal in signals {
        output_sum += calculate_output(signal);
    }

    println!("total output sum: {}", output_sum);
}

// Parse the input file to generate the list of signals.
fn parse_input(path: &str) -> Vec<Signal> {
    let file = match File::open(path) {
        Ok(handle) => handle,
        Err(err) => {
            panic!("invalid files {}: {}", path, err);
        }
    };

    let signals: Vec<Signal> = io::BufReader::new(file)
        .lines()
        .map(|x| match x {
            Ok(line) => {
                let tokens: Vec<String> = line.split(" | ").map(|x| x.to_string()).collect();
                if tokens.len() != 2 {
                    panic!("parsed {} tokens instead of 2", tokens.len());
                }

                let input = tokens[0].split(" ").map(|x| x.to_string()).collect();
                let output = tokens[1].split(" ").map(|x| x.to_string()).collect();

                Signal {
                    input: input,
                    output: output,
                }
            }

            Err(err) => {
                panic!("invalid line parsed: {}", err);
            }
        })
        .collect();

    signals
}

// Determine the number of unique digits in the signal outputs.
fn unique_output_digits(signals: &[Signal]) -> i32 {
    let mut count = 0;

    for signal in signals {
        for elem in &signal.output {
            if elem.len() == 2 || elem.len() == 3 || elem.len() == 4 || elem.len() == 7 {
                count += 1;
            }
        }
    }

    count
}

// Calculate the total value of the output digits after determining the values of the signal inputs.
fn calculate_output(signal: Signal) -> i64 {
    let mut codes = vec![HashSet::<char>::new(); 10];

    for elem in &signal.input {
        let digit: HashSet<char> = elem.chars().collect();
        match digit.len() {
            2 => codes[1] = digit,
            3 => codes[7] = digit,
            4 => codes[4] = digit,
            7 => codes[8] = digit,
            _ => continue,
        }
    }

    if codes[1].is_empty() || codes[4].is_empty() || codes[7].is_empty() || codes[8].is_empty() {
        panic!("failed to find the unique set of digits: {:?}", codes);
    }

    for elem in signal.input {
        let digit: HashSet<char> = elem.chars().collect();
        let l_diff: HashSet<char> = codes[4].difference(&codes[1]).map(|x| *x).collect();

        match digit.len() {
            5 => {
                if digit
                    .intersection(&codes[1])
                    .map(|x| *x)
                    .collect::<HashSet<char>>()
                    == codes[1]
                {
                    codes[3] = digit;
                } else if digit
                    .intersection(&l_diff)
                    .map(|x| *x)
                    .collect::<HashSet<char>>()
                    == l_diff
                {
                    codes[5] = digit;
                } else {
                    codes[2] = digit;
                }
            }
            6 => {
                if digit
                    .intersection(&codes[4])
                    .map(|x| *x)
                    .collect::<HashSet<char>>()
                    == codes[4]
                {
                    codes[9] = digit;
                } else if digit
                    .intersection(&l_diff)
                    .map(|x| *x)
                    .collect::<HashSet<char>>()
                    == l_diff
                {
                    codes[6] = digit;
                } else {
                    codes[0] = digit;
                }
            }
            _ => (),
        }
    }

    // The output is digits represented by signals, so the running total is Tn = (Tn-1 * 10) + digit.
    let mut total = 0;
    for elem in signal.output {
        let digit: HashSet<char> = elem.chars().collect();
        for index in 0..codes.len() {
            if digit == codes[index] {
                total = (total * 10) + index;
                break;
            }
        }
    }

    total as i64
}
