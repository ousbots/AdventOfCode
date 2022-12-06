use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
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

    let diagnostics: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| match x {
            Ok(cmd) => cmd,
            Err(err) => {
                println!("invalid command parsed: {}", err);
                "".to_string()
            }
        })
        .collect();

    if diagnostics.len() == 0 {
        println!(
            "need more than {} diagnostic values to compute power",
            diagnostics.len()
        );
        return;
    }

    let (gamma, epsilon) = calculate_power_rates(&diagnostics);
    println!("calculated binary gamma {} and epsilon {}", gamma, epsilon);

    let gamma_val = i32::from_str_radix(&*gamma, 2).unwrap();
    let epsilon_val = i32::from_str_radix(&*epsilon, 2).unwrap();
    println!(
        "converts to decimal values gamma {} and epsilon {} with total power {}",
        gamma_val,
        epsilon_val,
        gamma_val * epsilon_val
    );

    let (oxygen, scrubber) = calculate_life_support_rates(&diagnostics);
    println!(
        "found oxygen {} and CO2 scrubber {} codes",
        oxygen, scrubber
    );

    let oxygen_val = i32::from_str_radix(&*oxygen, 2).unwrap();
    let scrubber_val = i32::from_str_radix(&*scrubber, 2).unwrap();
    println!(
        "converts to decimal values oxygen {} and CO2 scrubber {} with life support rating {}",
        oxygen_val,
        scrubber_val,
        oxygen_val * scrubber_val
    );
}

// Calculates the gamma and  epsilon values from the vector of diagnostic codes. The gamma and
// epsilon values are derived from the most and least common digits of all the codes.
fn calculate_power_rates(codes: &Vec<String>) -> (String, String) {
    let mut ones = HashMap::<i32, i32>::new();
    let mut zero = HashMap::<i32, i32>::new();

    for code in codes {
        let mut index = 0;
        for digit in code.chars() {
            let value = match digit.to_digit(10) {
                Some(val) => val,
                _ => {
                    println!("invalid digit conversion: {}", digit);
                    continue;
                }
            };

            match value {
                0 => {
                    let entry = match zero.entry(index) {
                        Vacant(entry) => entry.insert(0),
                        Occupied(entry) => entry.into_mut(),
                    };

                    *entry += 1;
                }

                1 => {
                    let entry = match ones.entry(index) {
                        Vacant(entry) => entry.insert(0),
                        Occupied(entry) => entry.into_mut(),
                    };

                    *entry += 1;
                }

                _ => {
                    println!("bad digit {} found, skipping", value);
                    continue;
                }
            };

            index += 1;
        }
    }

    if ones.len() != zero.len() {
        println!(
            "mismatched ones ({}) and zeroes ({})",
            ones.len(),
            zero.len()
        );
        return ("".to_string(), "".to_string());
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for index in 0..ones.len() {
        let one_count: i32 = match ones.get(&(index as i32)) {
            Some(val) => *val,
            _ => 0,
        };

        let zero_count = match zero.get(&(index as i32)) {
            Some(val) => *val,
            _ => 0,
        };

        if one_count >= zero_count {
            gamma += "1";
            epsilon += "0";
        } else {
            gamma += "0";
            epsilon += "1";
        }
    }

    (gamma, epsilon)
}

// Determine the oxygen and CO2 scrubber rates from the diagnostic codes. The rates are determined
// by filtering for the code with the most and least common digits respectively.
fn calculate_life_support_rates(codes: &Vec<String>) -> (String, String) {
    let mut oxygen_list = codes.clone();
    let mut scrubber_list = codes.clone();

    let mut index = 0;
    while oxygen_list.len() > 1 {
        let mut ones = 0;
        let mut zeros = 0;

        for elem in &oxygen_list {
            match elem.chars().nth(index).unwrap() {
                '0' => zeros += 1,
                '1' => ones += 1,
                _ => println!("non-digit character found in {} at index {}", elem, index),
            }
        }

        if ones >= zeros {
            oxygen_list.retain(|x| x.chars().nth(index).unwrap() == '1');
        } else {
            oxygen_list.retain(|x| x.chars().nth(index).unwrap() == '0');
        }
        index += 1;
    }

    index = 0;
    while scrubber_list.len() > 1 {
        let mut ones = 0;
        let mut zeros = 0;

        for elem in &scrubber_list {
            match elem.chars().nth(index).unwrap() {
                '0' => zeros += 1,
                '1' => ones += 1,
                _ => println!("non-digit character found in {} at index {}", elem, index),
            }
        }

        if ones >= zeros {
            scrubber_list.retain(|x| x.chars().nth(index).unwrap() == '0');
        } else {
            scrubber_list.retain(|x| x.chars().nth(index).unwrap() == '1');
        }
        index += 1;
    }

    if oxygen_list.len() != 1 && scrubber_list.len() != 1 {
        println!(
            "filtered incorrectly oxygen {:?} scrubber {:?}",
            oxygen_list, scrubber_list
        );
        return ("".to_string(), "".to_string());
    }

    (oxygen_list[0].clone(), scrubber_list[0].clone())
}
