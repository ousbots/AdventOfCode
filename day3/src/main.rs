/// Checks the elves rucksacks for bad items and group badges by parsing an input file that
/// represents each rucksack.
/// Notes:
///   - Each line represents the two equal sized compartments of a single rucksack.
///   - Each rucksack has only one bad item which is exists in both compartments.
///   - The badges are determined by the common item found in every three rucksacks.
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Check the rucksacks for bad items.
fn ruck_check(path: &str) -> Result<(), Box<dyn Error>> {
    let mut bad_value = 0;
    let mut rucks = 0;
    let mut group: Vec<Vec<char>> = vec![];
    let mut badge_value = 0;

    for raw in BufReader::new(File::open(path)?).lines() {
        let line: Vec<char> = raw?.chars().collect();

        rucks += 1;
        bad_value += item_value(bad_item(&line));

        if group.len() < 3 {
            group.push(line);

            if group.len() == 3 {
                badge_value += item_value(group_badge(group.clone()));
                group.clear();
            }
        }
    }

    println!(
        "Checking {} rucksacks resulted in {} worth of bad items.",
        rucks, bad_value
    );

    println!("Badge group value {}", badge_value);

    Ok(())
}

// Find the duplicate item in the rucksacks.
// Note: each rucksack is the same length of characters.
fn bad_item(rucksacks: &Vec<char>) -> char {
    let middle = rucksacks.len() / 2;
    let mut letters: HashMap<char, bool> = HashMap::new();

    for i in 0..rucksacks.len() {
        if i < middle {
            letters.insert(rucksacks[i], true);
        } else {
            if letters.contains_key(&rucksacks[i]) {
                return rucksacks[i];
            }
        }
    }

    panic!("couldn't find a bad item in {:?}", rucksacks);
}

// Determines the group badge for a given set of rucksacks.
// Note: a group badge is the item that is common to all rucksacks.
fn group_badge(ruck_group: Vec<Vec<char>>) -> char {
    let mut seen: HashMap<char, i32> = HashMap::new();

    for item in ruck_group[0].iter() {
        seen.insert(*item, 1);
    }

    for item in ruck_group[1].iter() {
        let entry = seen.entry(*item).or_insert(0);
        if *entry == 1 {
            *entry = 2;
        }
    }

    for item in ruck_group[2].iter() {
        let entry = seen.entry(*item).or_insert(0);
        if *entry == 2 {
            *entry = 3;
        }
    }

    for (key, value) in seen.iter() {
        if *value == 3 {
            return *key;
        }
    }

    panic!("could not find a badge item: {:?}", seen);
}

// The value assigned to each item.
// Note: the values are ascending alphabetically:
//   a-z => 1 - 26
//   A-Z => 27 - 52
// BUG: this approach only works for ASCII encoded characters.
fn item_value(item: char) -> i32 {
    let mut value = item as i32;

    if value >= 97 && value <= 122 {
        value -= 96;
    } else if value >= 65 && value <= 90 {
        value -= 38;
    } else {
        panic!("{} is not a valid item", item);
    }

    value
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!(
            "expected only one argument: the path to the input file\n\tRecieved {:?}",
            &args[1..]
        );
    }
    let path = args[1].to_owned();

    ruck_check(&path).expect("failed while processing input");
}
