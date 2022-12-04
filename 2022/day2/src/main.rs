/// Simulates the game of Rock, Paper, Scissors by parsing an input file that represents a
/// gameplay strategy and calculating outcomes.
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

// The move played during a rock paper scissors game.
// Note: The moves are input by letters and have the values:
//   Rock => "A" or "X" =>  1
//   Paper => "B" or "Y" => 2
//   Scissors => "C" or "Z" => 3
#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn value(self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn parse(symbol: &str) -> Self {
        match symbol {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissors,
            _ => panic!("{} is an invalid move", symbol),
        }
    }
}

// The game outcome and points awarded.
// Note: the points awarded are:
//   Loss => 0
//   Draw => 3
//   Win => 6
#[derive(Clone, Copy, Debug)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn value(self) -> i32 {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }

    fn reverse(self) -> Self {
        match self {
            Self::Loss => Self::Win,
            Self::Draw => Self::Draw,
            Self::Win => Self::Loss,
        }
    }
}

// Generate the results of a game of Rock, Paper, Scissors.
// Note: standard rules
//   Rock beats Scissors
//   Paper beats Rock
//   Scissors beats Rock
fn play(player: &Shape, opponent: &Shape) -> Outcome {
    match player {
        Shape::Rock => match opponent {
            Shape::Rock => Outcome::Draw,
            Shape::Paper => Outcome::Loss,
            Shape::Scissors => Outcome::Win,
        },

        Shape::Paper => match opponent {
            Shape::Rock => Outcome::Win,
            Shape::Paper => Outcome::Draw,
            Shape::Scissors => Outcome::Loss,
        },

        Shape::Scissors => match opponent {
            Shape::Rock => Outcome::Loss,
            Shape::Paper => Outcome::Win,
            Shape::Scissors => Outcome::Draw,
        },
    }
}

fn simulate(path: &String) -> Result<(), Box<dyn Error>> {
    let mut results: Vec<(i32, i32)> = vec![];
    let (mut player, mut opponent) = (0, 0);

    let file = File::open(path)?;

    for raw in BufReader::new(file).lines() {
        let line = raw?;
        let moves: Vec<&str> = line.split(" ").collect();
        if moves.len() != 2 {
            panic!("input line {:?} is invalid", line);
        }

        let opponent_move = Shape::parse(&moves[0]);
        let player_move = Shape::parse(&moves[1]);

        let result = play(&player_move, &opponent_move);
        let player_score = player_move.value() + result.value();
        let opponent_score = opponent_move.value() + result.reverse().value();

        player += player_score;
        opponent += opponent_score;

        results.push((player_score, opponent_score));
    }

    println!(
        "final score after {} rounds:\n\tplayer: {}\n\topponent: {}",
        results.len(),
        player,
        opponent
    );

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!(
            "expected only one argument: the path to the input file\n\tRecieved {:?}",
            &args[1..]
        );
    }

    let path = args[1].to_owned();

    simulate(&path).expect("simulation failure");
}
