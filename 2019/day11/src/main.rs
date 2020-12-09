use std::fs;
use std::io;

fn paint(painted: &Vec<(i64, i64)>) {
    let (mut min_x, mut max_x): (i64, i64) = (0, 0);
    let (mut min_y, mut max_y): (i64, i64) = (0, 0);

    for (x, y) in painted {
        if *x < min_x {
            min_x = *x;
        }

        if *x > max_x {
            max_x = *x;
        }

        if *y < min_y {
            min_y = *y;
        }

        if *y > max_y {
            max_y = *y;
        }
    }

    for y in 0 .. min_y + max_y + 1 {
        for x in 0 .. min_x + max_x + 1 {
            let mut to_paint: bool = false;
            for (painted_x, painted_y) in painted {
                if x == *painted_x && y == *painted_y {
                    to_paint = true;
                    break;
                }
            }

            if to_paint {
                print!("#");
            }

            else {
                print!(".");
            }
        }

        print!("\n");
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn turn(&self, direction: i64) -> Direction {
        match direction {
            0 => {
                match self {
                    Direction::Up => return Direction::Left,
                    Direction::Down => return Direction::Right,
                    Direction::Left => return Direction::Down,
                    Direction::Right => return Direction::Up,
                }
            },

            1 => {
                match self {
                    Direction::Up => return Direction::Right,
                    Direction::Down => return Direction::Left,
                    Direction::Left => return Direction::Up,
                    Direction::Right => return Direction::Down,
                }
            },

            _ => panic!("bad direction {}", direction),
        }
    }
}

fn main() -> io::Result<()> {
    let mut cpu: intcode::Computer = intcode::new();

    // Parse the input file to a memory vector
    let mut input: String = fs::read_to_string("input")?;
    input.pop();

    let split = input.split(",");
    for element in split {
        if let Ok(num) = element.parse::<i64>() {
            cpu.program.push(num);
        }

        else {
            panic!("{} is not valid IntCode", element);
        }
    }

    // Add 2k memory
    for _ in 0 .. 2_000 {
        cpu.memory.push(0);
    }

    cpu.print_program();

    let mut painted_blocks: Vec<(i64, i64)> = vec![];
    let mut dir: Direction = Direction::Up;
    let (mut x, mut y): (i64, i64) = (0, 0);


    while cpu.halt == false {
        let mut painted: bool = false;
        for (x_paint, y_paint) in &painted_blocks {
            if x == *x_paint && y == *y_paint {
                painted = true;
                break;
            }
        }

        if painted {
            cpu.input.push(1);
        }

        else {
            cpu.input.push(0);
        }

        println!("cpu input {}", cpu.input[0]);

        cpu.run();

        if cpu.output.len() < 2 {
            panic!("insufficient output")
        }

        let to_paint: i64 = cpu.output[0];
        cpu.output.remove(0);

        let turn: i64 = cpu.output[0];
        cpu.output.remove(0);

        match to_paint {
            0 => (),
            1 => painted_blocks.push((x, y)),
            _ => panic!("bad paint value {}", to_paint),
        }

        dir = dir.turn(turn);

        match dir {
            Direction::Up => y += 1,
            Direction::Down => y -= 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }

        println!("position: {} {}, painting: {}, facing: {:?}", x, y, to_paint, dir);
        //println!("cpu:\n{:?}\n", cpu);
    }

    println!("painted blocks: {}", painted_blocks.len());
    paint(&painted_blocks);

    Ok(())
}
