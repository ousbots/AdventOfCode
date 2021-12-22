/// Brute force solution, can be done more efficiently.
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Status {
    Empty,
    Filled,
}

#[derive(Clone, Copy, Debug)]
struct Square {
    value: i32,
    status: Status,
}

type Board = Vec<Vec<Square>>;

fn main() {
    let (turns, boards) = parse_input("assets/input.txt");

    if turns.len() < 1 {
        panic!("more than {} turns required to play", turns.len());
    }

    if boards.len() < 2 {
        panic!("more than {} boards required to play", boards.len());
    }

    println!(
        "The winning score is {}",
        find_winning_score(&mut boards.clone(), &turns)
    );

    println!(
        "The losing score is {}",
        find_losing_score(&mut boards.clone(), &turns)
    );
}

// Parse the input file to generate the list of numbers called and the boards.
fn parse_input(path: &str) -> (Vec<i32>, Vec<Board>) {
    let file = match File::open(path) {
        Ok(handle) => handle,
        Err(err) => {
            panic!("invalid files {}: {}", path, err);
        }
    };

    let setup: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| match x {
            Ok(line) => line,
            Err(err) => {
                panic!("invalid line parsed: {}", err);
            }
        })
        .collect();

    // At least 7 lines: numbers, blank, x5 board.
    if setup.len() < 7 {
        panic!("{} bingo input lines is insufficient to play", setup.len());
    }

    let numbers: Vec<i32> = setup[0]
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut boards: Vec<Board> = vec![];

    let mut index = 2;
    while index + 4 < setup.len() {
        let mut board = Board::new();

        for line in &setup[index..index + 5] {
            let values: Vec<i32> = line
                .split(" ")
                .filter_map(|x| x.trim().parse::<i32>().ok())
                .collect();

            let mut row: Vec<Square> = vec![];
            for value in values {
                let square = Square {
                    value: value,
                    status: Status::Empty,
                };

                row.push(square);
            }

            board.push(row);
        }

        boards.push(board);
        index += 6;
    }

    (numbers, boards)
}

// Find the score (sum of unmarked squares * winning number) of the first board to win.
fn find_winning_score(boards: &mut [Board], turns: &[i32]) -> i32 {
    let mut lowest_count = turns.len() as i32;
    let mut lowest_index = 0;

    for index in 0..boards.len() {
        let count = process_board(&mut boards[index], turns);

        if count < lowest_count {
            lowest_count = count;
            lowest_index = index;
        }
    }

    let mut sum = 0;
    for row in boards[lowest_index].clone() {
        for col in row {
            if col.status == Status::Empty {
                sum += col.value;
            }
        }
    }

    let winning_turn = turns[(lowest_count as usize) - 1];

    println!(
        "The winning parameters are sum {} turn {}",
        sum, winning_turn
    );

    winning_turn * sum
}

// Find the score (sum of unmarked squares * winning number) of the last board to win.
fn find_losing_score(boards: &mut [Board], turns: &[i32]) -> i32 {
    let mut highest_count = 0;
    let mut highest_index = 0;

    for index in 0..boards.len() {
        let count = process_board(&mut boards[index], turns);

        if count > highest_count {
            highest_count = count;
            highest_index = index;
        }
    }

    let mut sum = 0;
    for row in boards[highest_index].clone() {
        for col in row {
            if col.status == Status::Empty {
                sum += col.value;
            }
        }
    }

    let losing_turn = turns[(highest_count as usize) - 1];

    println!("The losing parameters are sum {} turn {}", sum, losing_turn);

    losing_turn * sum
}

// Determines how many turns it takes a board to win and marks the positions.
fn process_board(board: &mut Board, turns: &[i32]) -> i32 {
    let mut count = 0;

    for turn in turns {
        count += 1;

        'check: for row in &mut *board {
            for col in row {
                if col.value == *turn {
                    col.status = Status::Filled;
                    break 'check;
                }
            }
        }

        if win_state(&board) {
            break;
        }
    }

    count
}

// Checks if a board is in a winning state. A winning state is when every element of a row or
// column has been matched.
fn win_state(board: &Board) -> bool {
    let mut columns = vec![true, true, true, true, true];

    for row in board {
        let mut win = true;
        for col in 0..row.len() {
            if row[col].status == Status::Empty {
                columns[col] = false;
                win = false;
            }
        }

        if win {
            return true;
        }
    }

    for win in columns {
        if win {
            return true;
        }
    }

    false
}
