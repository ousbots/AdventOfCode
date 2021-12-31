use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Fold {
    horiz: bool,
    point: usize,
}

// Parse the input file to generate the point map and fold list.
fn parse_input(path: &str) -> (Vec<Point>, Vec<Fold>) {
    let file = match File::open(path) {
        Ok(handle) => handle,
        Err(err) => {
            panic!("invalid files {}: {}", path, err);
        }
    };

    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let points: Vec<Point> = lines
        .iter()
        .filter(|line| line.contains(","))
        .map(|line| {
            let tokens: Vec<String> = line.split(",").map(|x| x.to_string()).collect();

            if tokens.len() != 2 {
                panic!("invalid point token found {:?}", tokens);
            }

            Point {
                x: tokens[0].parse::<usize>().unwrap(),
                y: tokens[1].parse::<usize>().unwrap(),
            }
        })
        .collect();

    let folds: Vec<Fold> = lines
        .iter()
        .filter(|line| line.contains("fold along"))
        .map(|line| {
            let tokens: Vec<String> = line
                //.remove_matches("fold along ") // unstable feature.
                [11..] // "fold along ".len() == 11
                .split("=")
                .map(|x| x.to_string())
                .collect();

            if tokens.len() != 2 {
                panic!("invalid fold token found {:?}", tokens);
            }

            Fold {
                horiz: tokens[0] == "y",
                point: tokens[1].parse::<usize>().unwrap(),
            }
        })
        .collect();

    (points, folds)
}

// Prints the map of points in a pretty manner for visualization.
fn print_points(points: &[Point]) {
    let mut x_max = 0;
    let mut y_max = 0;

    for point in points {
        if point.x > x_max {
            x_max = point.x;
        }

        if point.y > y_max {
            y_max = point.y
        }
    }

    let mut map: Vec<Vec<char>> = vec![vec!['.'; x_max + 1]; y_max + 1];

    for point in points {
        map[point.y][point.x] = '#';
    }

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{} ", map[y][x]);
        }
        print!("\n");
    }
}

// Calculates new point positions after a single fold.
fn fold_points(points: &mut Vec<Point>, fold: &Fold) {
    for index in 0..points.len() {
        if fold.horiz {
            if points[index].y > fold.point {
                points[index].y = (points[index].y as i64 - (2 * fold.point as i64)).abs() as usize;
            }
        } else {
            if points[index].x > fold.point {
                points[index].x = (points[index].x as i64 - (2 * fold.point as i64)).abs() as usize;
            }
        }
    }

    points.sort();
    points.dedup();
}

fn main() {
    let (mut points, folds) = parse_input("assets/input.txt");

    fold_points(&mut points, &folds[0]);
    println!("after folding {} points remain", points.len());

    for fold in &folds[1..] {
        fold_points(&mut points, fold);
    }
    println!("final instructions:");
    print_points(&points);
}
