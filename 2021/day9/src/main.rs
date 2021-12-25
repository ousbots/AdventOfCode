use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let heights = parse_input("assets/input.txt");

    let low_points = low_points(&heights);

    println!(
        "risk level {}",
        low_points
            .iter()
            .map(|(x, y)| heights[*x][*y] + 1)
            .sum::<i32>()
    );

    let mut basin_sizes = vec![];
    for low_point in low_points {
        let basin_points = basin_points(&heights, low_point.0, low_point.1);
        basin_sizes.push(basin_points.len() + 1);
    }

    basin_sizes.sort();
    let len = basin_sizes.len();

    println!(
        "product of the largest three basins is {}",
        basin_sizes[len - 1] * basin_sizes[len - 2] * basin_sizes[len - 3]
    );
}

// Parse the input file to generate the smoke height map.
fn parse_input(path: &str) -> Vec<Vec<i32>> {
    let file = match File::open(path) {
        Ok(handle) => handle,
        Err(err) => {
            panic!("invalid files {}: {}", path, err);
        }
    };

    let heights: Vec<Vec<i32>> = io::BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .split("")
                .filter_map(|num| num.to_string().parse::<i32>().ok())
                .collect()
        })
        .collect();

    if heights.len() == 0 || heights[0].len() == 0 {
        panic!("height map is empty");
    }

    heights
}

// Find the local minimum values that are lower than all adjacent values.
fn low_points(heights: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut lows = vec![];

    let x_max = heights.len();
    let y_max = heights[0].len();

    for x in 0..x_max {
        for y in 0..y_max {
            if x < x_max - 1 {
                if heights[x][y] >= heights[x + 1][y] {
                    continue;
                }

                if y < y_max - 1 {
                    if heights[x][y] >= heights[x + 1][y + 1] {
                        continue;
                    }
                }

                if y > 0 {
                    if heights[x][y] >= heights[x + 1][y - 1] {
                        continue;
                    }
                }
            }

            if x > 0 {
                if heights[x][y] >= heights[x - 1][y] {
                    continue;
                }

                if y > 0 {
                    if heights[x][y] >= heights[x - 1][y - 1] {
                        continue;
                    }
                }

                if y < y_max - 1 {
                    if heights[x][y] >= heights[x - 1][y + 1] {
                        continue;
                    }
                }
            }

            if y < y_max - 1 {
                if heights[x][y] >= heights[x][y + 1] {
                    continue;
                }
            }

            if y > 0 {
                if heights[x][y] >= heights[x][y - 1] {
                    continue;
                }
            }

            lows.push((x, y));
        }
    }

    lows
}

// Find all points in the "basin" that flow towards the given point. The solution does not expect
// checking diagonals for flow, so they are commented out.
fn basin_points(heights: &Vec<Vec<i32>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut basin_stack: Vec<(usize, usize)> = vec![(x, y)];
    let mut basin_points: HashSet<(usize, usize)> = HashSet::new();

    let x_max = heights.len();
    let y_max = heights[0].len();

    while basin_stack.len() > 0 {
        let check_point = basin_stack.pop().unwrap();

        let x = check_point.0;
        let y = check_point.1;

        if x < x_max - 1 {
            if heights[x][y] <= heights[x + 1][y] {
                if heights[x + 1][y] != 9 && !basin_points.contains(&(x + 1, y)) {
                    basin_stack.push((x + 1, y));
                    basin_points.insert((x + 1, y));
                }
            }

            //if y < y_max - 1 {
            //    if heights[x][y] <= heights[x + 1][y + 1] {
            //        if heights[x + 1][y + 1] != 9 && !basin_points.contains(&(x + 1, y + 1)) {
            //            basin_stack.push((x + 1, y + 1));
            //            basin_points.insert((x + 1, y + 1));
            //        }
            //    }
            //}

            //if y > 0 {
            //    if heights[x][y] <= heights[x + 1][y - 1] {
            //        if heights[x + 1][y - 1] != 9 && !basin_points.contains(&(x + 1, y - 1)) {
            //            basin_stack.push((x + 1, y - 1));
            //            basin_points.insert((x + 1, y - 1));
            //        }
            //    }
            //}
        }

        if x > 0 {
            if heights[x][y] <= heights[x - 1][y] {
                if heights[x - 1][y] != 9 && !basin_points.contains(&(x - 1, y)) {
                    basin_stack.push((x - 1, y));
                    basin_points.insert((x - 1, y));
                }
            }

            //if y > 0 {
            //    if heights[x][y] <= heights[x - 1][y - 1] {
            //        if heights[x - 1][y - 1] != 9 && !basin_points.contains(&(x - 1, y - 1)) {
            //            basin_stack.push((x - 1, y - 1));
            //            basin_points.insert((x - 1, y - 1));
            //        }
            //    }
            //}

            //if y < y_max - 1 {
            //    if heights[x][y] <= heights[x - 1][y + 1] {
            //        if heights[x - 1][y + 1] != 9 && !basin_points.contains(&(x - 1, y + 1)) {
            //            basin_stack.push((x - 1, y + 1));
            //            basin_points.insert((x - 1, y + 1));
            //        }
            //    }
            //}
        }

        if y < y_max - 1 {
            if heights[x][y] <= heights[x][y + 1] {
                if heights[x][y + 1] != 9 && !basin_points.contains(&(x, y + 1)) {
                    basin_stack.push((x, y + 1));
                    basin_points.insert((x, y + 1));
                }
            }
        }

        if y > 0 {
            if heights[x][y] <= heights[x][y - 1] {
                if heights[x][y - 1] != 9 && !basin_points.contains(&(x, y - 1)) {
                    basin_stack.push((x, y - 1));
                    basin_points.insert((x, y - 1));
                }
            }
        }
    }

    basin_points
        .iter()
        .map(|x| *x)
        .collect::<Vec<(usize, usize)>>()
}
