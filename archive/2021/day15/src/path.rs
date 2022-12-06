use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::map::*;

// Convenience function to check if an element is in the heap.
// Note: Inefficient.
fn in_heap(set: &BinaryHeap<Position>, target: Position) -> bool {
    let mut found = false;

    for pos in set {
        if target == *pos {
            found = true;
            break;
        }
    }

    found
}

// Shortest path discovery in the map between the start and end positions. Uses A* pathfinding.
pub fn shortest(start: Position, end: Position, map: &mut Vec<Vec<i32>>) -> Option<Vec<Position>> {
    let mut open_set = BinaryHeap::<Position>::from([start]);
    let mut came_from = HashMap::<Position, Position>::new();

    let mut cost = HashMap::<Position, i32>::from([(start, 0i32)]);

    while let Some(current) = open_set.pop() {
        if current == end {
            return Some(reconstruct_path(current, came_from, map.to_vec()));
        }

        for mut neighbor in neighbors(current, map) {
            let temp_cost: i32 = match cost.entry(current) {
                Occupied(entry) => *entry.get() + neighbor.cost,
                Vacant(_) => i32::MAX,
            };

            let neighbor_cost: i32 = match cost.entry(neighbor) {
                Occupied(entry) => *entry.get(),
                Vacant(_) => i32::MAX,
            };

            if temp_cost < neighbor_cost {
                came_from.insert(neighbor, current);
                cost.insert(neighbor, temp_cost);

                neighbor.cost = temp_cost + cost_estimate(start, map);

                if !in_heap(&open_set, neighbor) {
                    open_set.push(neighbor)
                }
            }
        }
    }

    None
}

// Calculate the total cost in a path without the starting cost.
pub fn cost(path: &Vec<Position>) -> i32 {
    let mut cost = 0;
    for index in 0..path.len() {
        let pos = path[index];
        if pos.x == 0 && pos.y == 0 {
            continue;
        }

        cost += pos.cost;
    }

    cost
}

// Print the map with the path highlighted.
pub fn pretty_print(map: &Vec<Vec<i32>>, path: &Vec<Position>) {
    let mut highlight = HashSet::<(usize, usize)>::new();

    for elem in path {
        highlight.insert((elem.x, elem.y));
    }

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            match highlight.get(&(x, y)) {
                Some(_) => print!("\x1b[35;1m{}\x1b[0m", map[y][x]),
                None => print!("{}", map[y][x]),
            }
        }

        print!("\n");
    }
}

// Build the path from the optimal reverse map.
pub fn reconstruct_path(
    parent: Position,
    reverse: HashMap<Position, Position>,
    map: Vec<Vec<i32>>,
) -> Vec<Position> {
    let mut current = Position {
        cost: map[parent.y][parent.x],
        x: parent.x,
        y: parent.y,
    };
    let mut path = vec![current];

    loop {
        current = match reverse.get(&current) {
            Some(pos) => *pos,
            None => break,
        };

        current.cost = map[current.y][current.x];

        path.push(current);
    }

    path
}
