use std::fs;

// Reads tickets from the given path.
fn load_tickets(path: String) -> Vec<String> {
    let file_content = fs::read_to_string(path).expect("failed to read file");
    let lines = file_content.lines().map(|l| l.to_string()).collect();

    lines
}

// Converts the given ticket to a seat id.
fn ticket_to_seat_id(ticket: &String) -> i32 {
    let tokens: Vec<char> = ticket.chars().collect();
    if tokens.len() != 10 {
        println!("incorrect ticket length {}", tokens.len());
        return -1;
    }

    let mut lo_row: i32 = 0;
    let mut hi_row: i32 = 127;
    let mut lo_col: i32 = 0;
    let mut hi_col: i32 = 7;
    //tokens.iter().map(|t| match t {
    for token in tokens {
        match token {
            'F' => hi_row -= ((hi_row - lo_row) / 2) + 1,
            'B' => lo_row += ((hi_row - lo_row) / 2) + 1,
            'L' => hi_col -= ((hi_col - lo_col) / 2) + 1,
            'R' => lo_col += ((hi_col - lo_col) / 2) + 1,
            _ => println!("bad ticket token {}", token),
        }
    }

    if lo_row != hi_row {
        println!("mismatched row {} {}", lo_row, hi_row);
        return -1;
    }

    if lo_col != hi_col {
        println!("mismatched col {} {}", lo_col, hi_col);
        return -1;
    }

    (lo_row * 8) + lo_col
}

// Returns the first encountered likely missing seat that is yours.
fn find_seat(seat_ids: Vec<i32>) -> Vec<i32> {
    // skip the first two rows with seat 16.
    let mut prev_id = 16;
    let mut next_id = 16;

    let mut possible: Vec<i32> = Vec::new();
    for id in seat_ids {
        if next_id - 1 > prev_id {
            while prev_id < next_id - 1 {
                prev_id += 1;
                possible.push(prev_id);
            }
        }

        prev_id = next_id;
        next_id = id;
    }

    possible
}

fn main() {
    let tickets = load_tickets("input.txt".to_string());

    let mut ticket_ids: Vec<i32> = Vec::new();
    for ticket in tickets {
        let ticket_id = ticket_to_seat_id(&ticket);
        ticket_ids.push(ticket_id)
    }
    ticket_ids.sort();

    println!("max ticket id {}", ticket_ids[ticket_ids.len() - 1]);
    println!("my seat id {:#?}", find_seat(ticket_ids));
}
