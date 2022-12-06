use std::fs;
use std::io;

const FACTOR: f64 = 1_000_000.0;

fn print_map(map: &Vec<Vec<bool>>) {
    for line in map {
        for point in line {
            match point {
                true => print!("*"),
                false => print!("."),
            }
        }

        print!("\n");
    }
}

fn scan_map(map: &Vec<Vec<bool>>) -> (usize, usize) {
    let mut max_count: u64 = 0;
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

    for ycoord in 0 .. map.len() {
        for xcoord in 0 .. map[0].len() {
            if map[ycoord][xcoord] == true {
                let count = viewable_asteroids(&map, xcoord, ycoord);
                println!("Can see {} asteroids from |{}, {}|", count, xcoord, ycoord);

                if count > max_count {
                    max_count = count;
                    max_x = xcoord;
                    max_y = ycoord;
                }
            }
        }
    }

    println!("Can see {} asteroids from |{}, {}|", max_count, max_x, max_y);

    return (max_x, max_y);
}

fn viewable_asteroids(map: &Vec<Vec<bool>>, x_pos: usize, y_pos: usize) -> u64 {
    let mut blocked_pos: Vec<i64> = Vec::new();
    let mut blocked_neg: Vec<i64> = Vec::new();
    let mut count: u64 = 0;

    let x: i64 = x_pos as i64;
    let y: i64 = y_pos as i64;
    let y_max: i64 = map.len() as i64;
    let x_max: i64 = map[0].len() as i64;

    for y_diff in -y_max .. y_max {
        for x_diff in -x_max .. x_max {
            if x_diff == 0 && y_diff == 0 {
                continue;
            }

            if x + x_diff >= x_max || x + x_diff < 0 {
                continue;
            }

            if y + y_diff >= y_max || y + y_diff < 0 {
                continue;
            }

            let mut pos = false;
            if y >= 0 {
                pos = true;
            }

            let tan: i64;
            if x_diff == 0 {
                if y_diff > 0 {
                    tan = FACTOR as i64;
                }

                else {
                    tan = -FACTOR as i64;
                }
            }

            else {
                tan = ((y_diff as f64 / x_diff as f64).tan() * FACTOR) as i64 ;
            }

            let mut block_it: bool = false;
            if pos {
                for block_ratio in blocked_pos.clone() {
                    if tan == block_ratio {
                        block_it = true;
                        break;
                    }
                }
            }

            else {
                for block_ratio in blocked_neg.clone() {
                    if tan == block_ratio {
                        block_it = true;
                        break;
                    }
                }
            }

            if block_it {
                continue;
            }

            match map[(y + y_diff) as usize][(x + x_diff) as usize] {
                true => {
                    count += 1;
                    if pos {
                        blocked_pos.push(tan);
                    }

                    else {
                        blocked_neg.push(tan);
                    }
                },

                false => (),
            }
        }
    }

    return count;
}


fn main() -> io::Result<()> {
    // Parse the input file to a map of [][]bool
    let input: String = fs::read_to_string("input")?;
    let mut map: Vec<Vec<bool>> = Vec::new();

    for line in input.lines() {
        let mut map_line: Vec<bool> = Vec::new();

        for point in line.chars() {
            match point {
                '#' => map_line.push(true),
                '.' => map_line.push(false),
                _ => panic!("bad map symbol: {}", point),
            }
        }

        map.push(map_line);
    }

    print_map(&map);
    scan_map(&map);

    Ok(())
}
