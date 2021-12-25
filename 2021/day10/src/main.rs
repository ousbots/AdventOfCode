use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Symbol {
    ParenO,
    ParenC,
    BrackO,
    BrackC,
    CurlyO,
    CurlyC,
    AngleO,
    AngleC,
}

impl Symbol {
    fn value_corrupt(&self) -> i64 {
        match self {
            Symbol::ParenO => 3,
            Symbol::ParenC => 3,
            Symbol::BrackO => 57,
            Symbol::BrackC => 57,
            Symbol::CurlyO => 1197,
            Symbol::CurlyC => 1197,
            Symbol::AngleO => 25137,
            Symbol::AngleC => 25137,
        }
    }

    fn value_incomplete(&self) -> i64 {
        match self {
            Symbol::ParenC => 1,
            Symbol::BrackC => 2,
            Symbol::CurlyC => 3,
            Symbol::AngleC => 4,
            _ => panic!("no incomplete value for {:?}", self),
        }
    }

    fn is_open(&self) -> bool {
        match self {
            Symbol::ParenO => true,
            Symbol::BrackO => true,
            Symbol::CurlyO => true,
            Symbol::AngleO => true,
            _ => false,
        }
    }

    fn is_close(&self) -> bool {
        match self {
            Symbol::ParenC => true,
            Symbol::BrackC => true,
            Symbol::CurlyC => true,
            Symbol::AngleC => true,
            _ => false,
        }
    }

    fn matches(&self, other: Symbol) -> bool {
        match self {
            Symbol::ParenO => {
                if other == Symbol::ParenC {
                    return true;
                }
            }
            Symbol::BrackO => {
                if other == Symbol::BrackC {
                    return true;
                }
            }
            Symbol::CurlyO => {
                if other == Symbol::CurlyC {
                    return true;
                }
            }
            Symbol::AngleO => {
                if other == Symbol::AngleC {
                    return true;
                }
            }
            _ => {
                println!("not opening symbol {:?}", self);
                return false;
            }
        }

        false
    }

    fn opposite(&self) -> Symbol {
        match self {
            Symbol::ParenO => Symbol::ParenC,
            Symbol::ParenC => Symbol::ParenO,
            Symbol::BrackO => Symbol::BrackC,
            Symbol::BrackC => Symbol::BrackO,
            Symbol::CurlyO => Symbol::CurlyC,
            Symbol::CurlyC => Symbol::CurlyO,
            Symbol::AngleO => Symbol::AngleC,
            Symbol::AngleC => Symbol::AngleO,
        }
    }

    fn from(symbol: char) -> Option<Symbol> {
        match symbol {
            '(' => Some(Symbol::ParenO),
            ')' => Some(Symbol::ParenC),
            '[' => Some(Symbol::BrackO),
            ']' => Some(Symbol::BrackC),
            '{' => Some(Symbol::CurlyO),
            '}' => Some(Symbol::CurlyC),
            '<' => Some(Symbol::AngleO),
            '>' => Some(Symbol::AngleC),
            _ => {
                println!("bad symbol {}", symbol);
                None
            }
        }
    }
}

// Parse the input file to generate the symbol lines.
fn parse_input(path: &str) -> Vec<Vec<Symbol>> {
    let file = match File::open(path) {
        Ok(handle) => handle,
        Err(err) => {
            panic!("invalid files {}: {}", path, err);
        }
    };

    let symbols: Vec<Vec<Symbol>> = io::BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .filter_map(|x| Symbol::from(x))
                .collect()
        })
        .collect();

    symbols
}

// Finds the first bad symbol in lines that have mismatched opening and closing symbols.
fn corrupted_lines(symbols: &Vec<Vec<Symbol>>) -> Vec<(usize, Symbol)> {
    let mut bad_symbols: Vec<(usize, Symbol)> = vec![];

    for (index, line) in symbols.iter().enumerate() {
        let mut stack: Vec<Symbol> = vec![];

        for symbol in line {
            if symbol.is_open() {
                stack.push(*symbol);
                continue;
            }

            if symbol.is_close() {
                let check = stack.pop().unwrap();

                if !check.matches(*symbol) {
                    bad_symbols.push((index, *symbol));
                    break;
                }
            }
        }
    }

    bad_symbols
}

// Find the symbols that are missing to close the incomplete lines.
fn missing_symbols(symbols: &Vec<Vec<Symbol>>) -> Vec<Vec<Symbol>> {
    let mut missing = vec![];

    for line in symbols {
        let mut stack = vec![];

        for symbol in line {
            if symbol.is_open() {
                stack.push(*symbol);
                continue;
            }

            if symbol.is_close() {
                let check = stack.pop().unwrap();

                if !check.matches(*symbol) {
                    println!("unmatched symbol {:?}", *symbol);
                }
            }
        }

        if stack.len() > 0 {
            missing.push(stack);
        }
    }

    missing
        .iter()
        .map(|line| line.iter().rev().map(|sym| sym.opposite()).collect())
        .collect::<Vec<Vec<Symbol>>>()
}

fn main() {
    let mut lines = parse_input("assets/input.txt");

    let mut corrupted_sum = 0;
    let corrupted = corrupted_lines(&lines);

    let mut bad_lines = vec![];
    for symbol in corrupted {
        bad_lines.push(symbol.0);
        corrupted_sum += symbol.1.value_corrupt();
    }
    println!("corrupted sum {}", corrupted_sum);

    // Remove corrupted lines.
    bad_lines.sort();
    bad_lines.reverse();

    for index in bad_lines {
        lines.remove(index);
    }

    let missing_symbols = missing_symbols(&lines);

    let mut incomplete_scores = vec![];
    for line in missing_symbols {
        let mut incomplete_sum = 0;
        for symbol in line {
            incomplete_sum = (incomplete_sum * 5) + symbol.value_incomplete();
        }
        incomplete_scores.push(incomplete_sum);
    }
    incomplete_scores.sort();

    println!(
        "incomplete score winner {:?}",
        incomplete_scores[incomplete_scores.len() / 2]
    );
}
