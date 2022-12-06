use std::fs;
use std::io;
use std::collections::HashMap;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Collision {
    x: i64,
    y: i64,
    steps: i64,
}

const DEBUG: bool = false;

fn main() -> io::Result<()> {
    let input: String = fs::read_to_string("input")?;
    let lines = input.split("\n");
    let mut wires: Vec<Vec<String>> = Vec::new();

    for element in lines {
        let split = element.split(",");
        let mut instrs: Vec<String> = Vec::new();

        for instr in split {
            if instr == "\n" {
                continue;
            }

            instrs.push(instr.to_string());
        }

        wires.push(instrs);
    }

    let mut collisions: Vec<Collision> = Vec::new();
    let mut positions: HashMap<Point, i64> = HashMap::new();

    let mut first = true;

    for wire in wires {
        let mut x: i64 = 0;
        let mut y: i64 = 0;
        let mut count: i64 = 0;

        for instr in wire {
            let chars: Vec<char> = instr.chars().collect();

            if chars.len() == 0 {
                continue;
            }

            let dir = chars[0];
            let len_str: String = chars[1 ..].iter().collect();
            let mut len = len_str.parse::<i64>().unwrap();

            match dir {
                'U' => {
                    while len > 0 {
                        count += 1;
                        y += 1;
                        let point = Point{x, y};
                        match positions.get(&point) {
                            Some(steps) => {
                                if DEBUG {
                                    println!("collision: {} {} ({})", x, y, instr);
                                }

                                if !first {
                                    collisions.push(Collision{x, y, steps: count+steps});
                                }
                            },
                            None => {
                                if DEBUG {
                                    println!("no collision: {} {} ({})", x, y, instr);
                                }

                                if first {
                                    positions.insert(point, count);
                                }
                            },
                        }

                        len -= 1;
                    }
                },

                'D' => {
                    while len > 0 {
                        count += 1;
                        y -= 1;
                        let point = Point{x, y};
                        match positions.get(&point) {
                            Some(steps) => {
                                if DEBUG {
                                    println!("collision: {} {} ({})", x, y, instr);
                                }

                                if !first {
                                    collisions.push(Collision{x, y, steps: count+steps});
                                }
                            },
                            None => {
                                if DEBUG {
                                    println!("no collision: {} {} ({})", x, y, instr);
                                }

                                if first {
                                    positions.insert(point, count);
                                }
                            },
                        }

                        len -= 1;
                    }
                },

                'R' => {
                    while len > 0 {
                        count += 1;
                        x += 1;
                        let point = Point{x, y};
                        match positions.get(&point) {
                            Some(steps) => {
                                if DEBUG {
                                    println!("collision: {} {} ({})", x, y, instr);
                                }

                                if !first {
                                    collisions.push(Collision{x, y, steps: count+steps});
                                }
                            },
                            None => {
                                if DEBUG {
                                    println!("no collision: {} {} ({})", x, y, instr);
                                }

                                if first {
                                    positions.insert(point, count);
                                }
                            },
                        }

                        len -= 1;
                    }
                },

                'L' => {
                    while len > 0 {
                        count += 1;
                        x -= 1;
                        let point = Point{x, y};
                        match positions.get(&point) {
                            Some(steps) => {
                                if DEBUG {
                                    println!("collision: {} {} ({})", x, y, instr);
                                }

                                if !first {
                                    collisions.push(Collision{x, y, steps: count+steps});
                                }
                            },
                            None => {
                                if DEBUG {
                                    println!("no collision: {} {} ({})", x, y, instr);
                                }

                                if first {
                                    positions.insert(point, count);
                                }
                            },
                        }

                        len -= 1;
                    }
                },

                _ => {
                    println!("strange instruction {}", instr);
                }
            }

        }

        first = false;
    }

    if collisions.len() == 0 {
        println!("no collisions!");
        return Ok(())
    }

    let mut closest_intersect = &collisions[0];
    for elem in collisions[1 ..].iter() {
        if elem.x.abs() + elem.y.abs() < closest_intersect.x.abs() + closest_intersect.y.abs() {
            closest_intersect = elem;
        }
    }

    let mut min_intersect = &collisions[0];
    for elem in collisions[1 ..].iter() {
        if elem.steps < min_intersect.steps {
            min_intersect = elem;
        }
    }

    println!("closest collision: {:?} {}", closest_intersect, closest_intersect.x.abs() + closest_intersect.y.abs());
    println!("minimum collisions: {:?} {}", min_intersect, min_intersect.steps);

    Ok(())
}
