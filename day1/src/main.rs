use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Note: this was unecessary, I thought part 2 would use this data.
struct Inventory {
    total: i32,
    calories: Vec<i32>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("assets/input.txt").expect("couldn't open input file");

    let mut inventory: Vec<Inventory> = vec![];
    let mut first = 0;
    let mut second = 0;
    let mut third = 0;

    for raw in BufReader::new(file).lines() {
        if let Ok(line) = raw {
            if line == "" || inventory.len() == 0 {
                inventory.push(Inventory {
                    total: 0,
                    calories: vec![],
                });

                if line == "" {
                    continue;
                }
            }

            let calories = line.parse()?;

            let last = inventory.last_mut().ok_or("empty list")?;

            last.total += calories;
            last.calories.push(calories);

            if last.total > third {
                if last.total > second {
                    if last.total > first {
                        third = second;
                        second = first;
                        first = last.total;
                    } else {
                        third = second;
                        second = last.total;
                    }
                } else {
                    third = last.total;
                }
            }
        }
    }

    println!(
        "highest calorie inventories: {} {} {}",
        first, second, third
    );
    println!("total top 3 calories: {}", first + second + third);

    Ok(())
}
