use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Segment {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}) -> ({}, {})",
            self.x1, self.y1, self.x2, self.y2
        )
    }
}

fn main() {
    let segments = parse_input("assets/input.txt");

    let overlaps_no_diag = calculate_segment_overlaps(&segments, true);
    println!(
        "Found {} overlapping non-diagonal line segments",
        overlaps_no_diag
    );

    let overlaps = calculate_segment_overlaps(&segments, false);
    println!("Found {} overlapping line segments", overlaps);
}

// Parse the input file to generate the list of line segments.
fn parse_input(path: &str) -> Vec<Segment> {
    let file = match File::open(path) {
        Ok(handle) => handle,
        Err(err) => {
            panic!("invalid files {}: {}", path, err);
        }
    };

    let segments: Vec<Segment> = io::BufReader::new(file)
        .lines()
        .map(|x| match x {
            Ok(line) => {
                let points: Vec<String> = line.split(" -> ").map(|x| x.to_string()).collect();

                if points.len() != 2 {
                    panic!("invalid number of points found: {}", points.len());
                }

                let point_a: Vec<String> = points[0].split(",").map(|x| x.to_string()).collect();
                let point_b: Vec<String> = points[1].split(",").map(|x| x.to_string()).collect();

                Segment {
                    x1: point_a[0].parse::<i32>().unwrap(),
                    y1: point_a[1].parse::<i32>().unwrap(),
                    x2: point_b[0].parse::<i32>().unwrap(),
                    y2: point_b[1].parse::<i32>().unwrap(),
                }
            }
            Err(err) => {
                panic!("invalid line parsed: {}", err);
            }
        })
        .collect();

    if segments.len() == 0 {
        panic!(
            "{} line segments is insufficient for vent calculations",
            segments.len()
        );
    }

    segments
}

// Calculate the number of points that have overlapping line segments. Skipping the diagonal
// segments is optional.
fn calculate_segment_overlaps(segments: &[Segment], skip_diag: bool) -> i32 {
    let mut points: HashMap<(i32, i32), i32> = HashMap::new();

    for segment in segments {
        if skip_diag && segment.x1 != segment.x2 && segment.y1 != segment.y2 {
            println!("skipping diagonal segment {}", segment);
            continue;
        }

        let (x_min, x_max, y_min, y_max);

        if segment.x1 > segment.x2 {
            x_min = segment.x2;
            y_min = segment.y2;
            x_max = segment.x1;
            y_max = segment.y1;
        } else {
            x_min = segment.x1;
            y_min = segment.y1;
            x_max = segment.x2;
            y_max = segment.y2;
        }

        let y_increment = if y_max > y_min { 1 } else { -1 };

        let mut x_val = x_min;
        let mut y_val = y_min;

        loop {
            let entry = match points.entry((x_val, y_val)) {
                Vacant(entry) => entry.insert(0),
                Occupied(entry) => entry.into_mut(),
            };

            *entry += 1;

            if x_val == x_max && y_val == y_max {
                break;
            }

            if x_val < x_max {
                x_val += 1;
            }

            if y_val != y_max {
                y_val += y_increment;
            }
        }
    }

    let mut overlaps = 0;
    for (point, count) in &points {
        if *count > 1 {
            overlaps += 1;
        }
    }

    overlaps
}
