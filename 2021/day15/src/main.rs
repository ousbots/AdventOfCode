mod map;
mod path;

use map::*;
use path::*;

fn main() {
    let mut map = parse("assets/input.txt");

    let mut x_max = map[0].len() - 1;
    let mut y_max = map.len() - 1;

    let start = Position {
        cost: map[0][0],
        x: 0,
        y: 0,
    };

    let mut end = Position {
        cost: map[y_max][x_max],
        x: x_max,
        y: y_max,
    };

    let path = shortest(start, end, &mut map).unwrap();
    pretty_print(&map, &path);
    println!("total cost {} in {} x {} map", cost(&path), x_max, y_max);

    let mut real_map = expand(map);
    x_max = real_map[0].len() - 1;
    y_max = real_map.len() - 1;
    end.cost = real_map[y_max][x_max];
    end.x = x_max;
    end.y = y_max;

    let real_path = shortest(start, end, &mut real_map).unwrap();
    pretty_print(&real_map, &real_path);
    println!(
        "total cost {} in {} x {} map",
        cost(&real_path),
        x_max,
        y_max
    );
}
