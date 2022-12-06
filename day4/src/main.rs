/// Checks the elves section cleaning assignment list to determine the number of overlapping
/// section cleaning pairs.
/// Notes:
///   - Each input line represents a cleaning assignment pair "a-b,x-y" where the first elf
///     is assigned sections a-b and the second elf sections x-y.
///   - Counts for both complete (part 1) and partial (part 2) overlapping are required.
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Process the input file to determine the number of cleaning assignment pairs that overlap.
fn find_overlap(path: String) -> Result<(), Box<dyn Error>> {
    let mut full_overlap = 0;
    let mut partial_overlap = 0;

    for raw in BufReader::new(File::open(path)?).lines() {
        let line = raw?;

        let spans: Vec<&str> = line.split(",").collect();
        if spans.len() != 2 {
            panic!("invalid input line: {}", line);
        }

        let first: Vec<&str> = spans[0].split("-").collect();
        if spans.len() != 2 {
            panic!("invalid first span in line: {}", line);
        }

        let second: Vec<&str> = spans[1].split("-").collect();
        if spans.len() != 2 {
            panic!("invalid second span in line: {}", line);
        }

        let inita: u32 = first[0].parse()?;
        let enda: u32 = first[1].parse()?;
        let initb: u32 = second[0].parse()?;
        let endb: u32 = second[1].parse()?;

        if inita < initb {
            if enda >= initb {
                partial_overlap += 1;
            }
            if enda >= endb {
                full_overlap += 1;
            }
        } else if initb < inita {
            if endb >= inita {
                partial_overlap += 1;
            }
            if endb >= enda {
                full_overlap += 1;
            }
        } else {
            full_overlap += 1;
            partial_overlap += 1;
        }
    }

    println!(
        "{} assignment pairs completely contain the other",
        full_overlap
    );
    println!(
        "{} assignment pairs partially contain the other",
        partial_overlap
    );

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!(
            "expected only the input file as an argument, not {:?}",
            &args[1..]
        );
    }

    find_overlap(args[1].to_owned()).expect("failed to process input");
}
