/// Simulates the game of Rock, Paper, Scissors (RPS) by parsing an input file that represents a
/// gameplay strategy and calculating outcomes.
/// Notes:
///   - The simulation uses standard RPS rules: Rock beats Scissors, Paper beats Rock, and Scissors
///     beats Paper.
///   - The first part of the problem is generating the results from the input being column one as
///     the opponent's shape and column two as the player's shape.
///   - The second part of the problem is generating the results from from the input being column
///     one as the opponent's shape and column two as the desired result, which adds calculating
///     the required move.
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
    // The point value assigned to an RPS shape.
    fn value(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    // Parse a string symbol into an RPS shape.
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

    // Generate the results of a game of RPS against the opponent's played shape.
    fn play(&self, opponent: &Shape) -> Outcome {
        match self {
            Self::Rock => match opponent {
                Self::Rock => Outcome::Draw,
                Self::Paper => Outcome::Loss,
                Self::Scissors => Outcome::Win,
            },
            Self::Paper => match opponent {
                Self::Rock => Outcome::Win,
                Self::Paper => Outcome::Draw,
                Self::Scissors => Outcome::Loss,
            },
            Self::Scissors => match opponent {
                Self::Rock => Outcome::Loss,
                Self::Paper => Outcome::Win,
                Self::Scissors => Outcome::Draw,
            },
        }
    }

    // Generate the shape that the player needs to play in a game of RPS to achieve the desired
    // result.
    // Note: The self shape is the opponent's move, but the desired result is from the perspevtive
    //   of the generated move.
    fn solve(&self, result: &Outcome) -> Self {
        match self {
            Self::Rock => match result {
                Outcome::Loss => Self::Scissors,
                Outcome::Draw => Self::Rock,
                Outcome::Win => Self::Paper,
            },
            Self::Paper => match result {
                Outcome::Loss => Self::Rock,
                Outcome::Draw => Self::Paper,
                Outcome::Win => Self::Scissors,
            },
            Self::Scissors => match result {
                Outcome::Loss => Self::Paper,
                Outcome::Draw => Self::Scissors,
                Outcome::Win => Self::Rock,
            },
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
    // The point value assigned to an outcome.
    fn value(&self) -> i32 {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }

    // The opposite of an outcome.
    fn reverse(&self) -> Self {
        match self {
            Self::Loss => Self::Win,
            Self::Draw => Self::Draw,
            Self::Win => Self::Loss,
        }
    }

    // Parse a string symbol into an outcome.
    fn parse(symbol: &str) -> Self {
        match symbol {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("{} is an invalid outcome", symbol),
        }
    }
}

fn simulate(path: &String) -> Result<(), Box<dyn Error>> {
    let mut rounds = 0;
    let (mut player_one, mut opponent_one) = (0, 0);
    let (mut player_two, mut opponent_two) = (0, 0);

    let file = File::open(path)?;

    for raw in BufReader::new(file).lines() {
        let line = raw?;
        rounds += 1;

        let moves: Vec<&str> = line.split(" ").collect();
        if moves.len() != 2 {
            panic!("input line {:?} is invalid", line);
        }

        // Part one parsing:
        //   - column one is the opponent's move.
        //   - column two is the player's move.
        // Part two parsing:
        //   - column one is the opponent's move.
        //   - column two is the desired outcome.
        let opponent_move = Shape::parse(&moves[0]);
        let player_move_one = Shape::parse(&moves[1]);
        let desired_outcome = Outcome::parse(&moves[1]);

        // The results from part one are generated by determining the outcome of each game and
        // summing and point values.
        let result_one = &player_move_one.play(&opponent_move);
        player_one += player_move_one.value() + result_one.value();
        opponent_one += opponent_move.value() + result_one.reverse().value();

        // The results from part two are generated by determining the correct move to be played
        // in each game then summing the point values.
        let player_move_two = opponent_move.solve(&desired_outcome);
        player_two += desired_outcome.value() + player_move_two.value();
        opponent_two += opponent_move.value() + desired_outcome.reverse().value();
    }

    println!(
        "Part 1:\n\tfinal score after {} rounds:\n\t\tplayer: {}\n\t\topponent: {}",
        rounds, player_one, opponent_one,
    );

    println!(
        "Part 2:\n\tfinal score after {} rounds:\n\t\tplayer: {}\n\t\topponent: {}",
        rounds, player_two, opponent_two,
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
