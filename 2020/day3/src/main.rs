use std::fs;

// Holds the terrain map.
struct Map {
    terrain: Vec<Vec<bool>>,
}

impl Map {
    // Check if a position hits a tree. Takes into account "arboreal physics" where the terrain
    // repeats to the sides.
    fn hit_tree(&self, x: usize, y: usize) -> bool {
        if y >= self.terrain.len() {
            return false;
        }

        self.terrain[y][x % self.terrain[y].len()]
    }

    // check the number of trees hit by traversing the map.
    fn check_slope(&self, x_slope: usize, y_slope: usize) -> usize {
        let mut trees: usize = 0;

        let mut x: usize = 0;
        let mut y: usize = 0;

        while y < self.terrain.len() {
            if self.hit_tree(x, y) {
                trees += 1;
            }

            x += x_slope;
            y += y_slope;
        }

        trees
    }

    // check the number of trees hit in the initial problem slope of -1/3.
    fn part_one(&self) -> usize {
        self.check_slope(3, 1)
    }

    // check the number of trees hit on each of the given slopes multiplied together.
    // slopes to check are: -1/1, -1/3, -1/5, -1/7, -2/1
    fn part_two(&self) -> usize {
        self.check_slope(1, 1)
            * self.check_slope(3, 1)
            * self.check_slope(5, 1)
            * self.check_slope(7, 1)
            * self.check_slope(1, 2)
    }
}

// load the puzzle map from the given file. Returns a 2D vector of booleans representing the map.
// Trees are represented by true and open squares by false.
fn load_map(path: String) -> Map {
    let file_content = fs::read_to_string(path).expect("failed to read file");
    let lines = file_content.lines();

    let mut map: Vec<Vec<bool>> = Vec::new();
    for line in lines {
        let mut row: Vec<bool> = Vec::new();
        for point in line.chars() {
            match point {
                '.' => row.push(false),
                '#' => row.push(true),
                _ => println!("bad input {}", point),
            }
        }

        map.push(row)
    }

    Map { terrain: map }
}

fn main() {
    let path = std::env::args().nth(1).expect("path argument missing");
    let map = load_map(path);

    println!("hit {} trees", map.part_one());
    println!("hit {} more trees", map.part_two());
}
