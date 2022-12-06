use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
pub struct Target {
    pub x_min: i64,
    pub x_max: i64,
    pub y_min: i64,
    pub y_max: i64,
}

#[derive(Debug, PartialEq)]
pub struct Probe {
    pub x_pos: i64,
    pub y_pos: i64,
    pub x_vel: i64,
    pub y_vel: i64,
}

// Generate target values from the input file.
pub fn parse_file(path: &str) -> Target {
    let file = File::open(path).unwrap();

    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    if lines.len() != 1 {
        panic!("{} lines found", lines.len());
    }

    parse(&lines[0])
}

// Generate the target values from a string.
pub fn parse(data: &str) -> Target {
    let words: Vec<&str> = data.split(" ").collect();

    if words.len() != 4 {
        panic!("{} words found instead of 4: {:?}", words.len(), words);
    }

    if words[0] != "target" && words[1] != "area:" {
        panic!("unknown line found {}", data);
    }

    let x_vals: Vec<&str> = words[2].split("=").collect::<Vec<&str>>()[1]
        .split("..")
        .collect();
    let y_vals: Vec<&str> = words[3].split("=").collect::<Vec<&str>>()[1]
        .split("..")
        .collect();

    let mut x_max = x_vals[1].chars();
    x_max.next_back();

    Target {
        x_min: x_vals[0].parse::<i64>().unwrap(),
        x_max: x_max.as_str().parse::<i64>().unwrap(),
        y_min: y_vals[0].parse::<i64>().unwrap(),
        y_max: y_vals[1].parse::<i64>().unwrap(),
    }
}

// Checks if the probe is in the target area.
pub fn in_target(probe: &Probe, target: &Target) -> bool {
    if probe.x_pos >= target.x_min
        && probe.x_pos <= target.x_max
        && probe.y_pos >= target.y_min
        && probe.y_pos <= target.y_max
    {
        return true;
    }

    false
}

// Checks if the probe has passed the target and cannot hit it anymore.
pub fn missed(probe: &Probe, target: &Target) -> bool {
    if probe.x_vel >= 0 {
        if probe.x_pos > target.x_max {
            return true;
        }
    }

    if probe.x_vel <= 0 {
        if probe.x_pos < target.x_min {
            return true;
        }
    }

    if probe.y_vel <= 0 {
        if probe.y_pos < target.y_min {
            return true;
        }
    }

    false
}

// Updates the probe position and velocity for a single time step.
pub fn update(probe: &mut Probe) {
    probe.x_pos += probe.x_vel;
    probe.y_pos += probe.y_vel;

    if probe.x_vel > 0 {
        probe.x_vel -= 1;
    }

    if probe.x_vel < 0 {
        probe.x_vel += 1;
    }

    probe.y_vel -= 1;
}

// Brute force checks a set of starting probes for the highest one
pub fn find_highest_launch(target: &Target) -> i64 {
    let mut highest = 0;

    for x in target.x_min..target.x_max {
        for y in target.y_min..target.y_max {
            let mut probe = Probe {
                x_pos: 0,
                y_pos: 0,
                x_vel: x,
                y_vel: y,
            };

            while !missed(&probe, &target) {
                if probe.y_pos > highest {
                    highest = probe.y_pos;
                }

                update(&mut probe);
            }
        }
    }

    highest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        assert_eq!(
            parse("target area: x=1..14, y=15..29"),
            Target {
                x_min: 1,
                x_max: 14,
                y_min: 15,
                y_max: 29
            }
        );
    }

    #[test]
    fn in_target_test() {
        let mut probe = Probe {
            x_pos: 0,
            y_pos: 0,
            x_vel: 0,
            y_vel: 0,
        };

        let mut target = Target {
            x_min: 1,
            x_max: 2,
            y_min: 1,
            y_max: 2,
        };

        assert_eq!(in_target(&probe, &target), false);

        probe.x_pos = 1;
        assert_eq!(in_target(&probe, &target), false);

        probe.y_pos = 1;
        assert_eq!(in_target(&probe, &target), true);

        probe.x_pos = 2;
        assert_eq!(in_target(&probe, &target), true);

        probe.y_pos = 2;
        assert_eq!(in_target(&probe, &target), true);

        target.x_max = 3;
        target.y_max = 3;
        assert_eq!(in_target(&probe, &target), true);

        probe.x_pos = 4;
        probe.y_pos = 4;
        assert_eq!(in_target(&probe, &target), false);
    }

    #[test]
    fn missing_test() {
        let mut probe = Probe {
            x_pos: 0,
            y_pos: 0,
            x_vel: 1,
            y_vel: 1,
        };

        let target = Target {
            x_min: 1,
            x_max: 2,
            y_min: 1,
            y_max: 2,
        };

        assert_eq!(missed(&probe, &target), false);

        probe.x_pos = 3;
        assert_eq!(missed(&probe, &target), true);

        probe.x_pos = 2;
        assert_eq!(missed(&probe, &target), false);

        probe.y_pos = -1;
        probe.y_vel = 0;
        assert_eq!(missed(&probe, &target), true);
    }
}
