use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead};

use crate::path;

#[derive(Clone, Copy, Debug, Eq)]
pub struct Position {
    pub cost: i32,
    pub x: usize,
    pub y: usize,
}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Ord for Position {
    // Reverse comparison to enable min-heap.
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cost > other.cost {
            return Ordering::Less;
        }

        if self.cost < other.cost {
            return Ordering::Greater;
        }

        Ordering::Equal
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

// Parse the input file and generate the chiton risk map.
pub fn parse(path: &str) -> Vec<Vec<i32>> {
    let file = File::open(path).unwrap();

    io::BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .collect::<Vec<char>>()
                .iter()
                .map(|e| e.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

// Expand the map five times in each direction, increasing the values with the expansions.
pub fn expand(map: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut new_map = Vec::<Vec<i32>>::new();

    for y_add in 0..5 {
        for y in 0..map.len() {
            let mut row = Vec::<i32>::new();
            for x_add in 0..5 {
                for x in 0..map[0].len() {
                    let mut new_value = map[y][x] + y_add + x_add;

                    if new_value > 9 {
                        new_value = new_value % 9;
                    }

                    row.push(new_value);
                }
            }
            new_map.push(row);
        }
    }

    new_map
}

// Generates a list of neighbors.
pub fn neighbors(initial: Position, map: &mut Vec<Vec<i32>>) -> Vec<Position> {
    let mut neighbors = Vec::<Position>::new();

    if initial.x > 0 {
        neighbors.push(Position {
            cost: map[initial.y][initial.x - 1],
            x: initial.x - 1,
            y: initial.y,
        });
    }

    if initial.y > 0 {
        neighbors.push(Position {
            cost: map[initial.y - 1][initial.x],
            x: initial.x,
            y: initial.y - 1,
        });
    }

    if initial.x < map[0].len() - 1 {
        neighbors.push(Position {
            cost: map[initial.y][initial.x + 1],
            x: initial.x + 1,
            y: initial.y,
        });
    }

    if initial.y < map.len() - 1 {
        neighbors.push(Position {
            cost: map[initial.y + 1][initial.x],
            x: initial.x,
            y: initial.y + 1,
        });
    }

    neighbors
}

// Estimate heuristic of traversing y then x used in A* pathfinding.
pub fn cost_estimate(start: Position, map: &Vec<Vec<i32>>) -> i32 {
    let mut cost = start.cost;

    let y_max = map.len() - 1;
    let x_max = map[y_max].len() - 1;

    for y in start.y..=y_max {
        cost += map[y][start.x];
    }

    for x in start.x..=x_max {
        cost += map[y_max][x];
    }

    cost
}
