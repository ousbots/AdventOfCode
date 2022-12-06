use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
pub struct Token {
    pub version: i64,
    pub class: i64,
    pub value: i64,
}

// Generate packets from the input file.
pub fn parse_file(path: &str) -> Vec<Token> {
    let file = File::open(path).unwrap();

    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    if lines.len() != 1 {
        panic!("{} lines found", lines.len());
    }

    parse(&lines[0])
}

// Generate packets from a transmission string.
pub fn parse(data: &String) -> Vec<Token> {
    parse_packets(&mut convert(data))
}

// Convert from a hexadecimal string to a bit array.
fn convert(data: &String) -> Vec<u8> {
    let mut packets = Vec::<u8>::new();

    for num in data.chars() {
        let packet = match num {
            '0' => [0, 0, 0, 0],
            '1' => [0, 0, 0, 1],
            '2' => [0, 0, 1, 0],
            '3' => [0, 0, 1, 1],
            '4' => [0, 1, 0, 0],
            '5' => [0, 1, 0, 1],
            '6' => [0, 1, 1, 0],
            '7' => [0, 1, 1, 1],
            '8' => [1, 0, 0, 0],
            '9' => [1, 0, 0, 1],
            'A' => [1, 0, 1, 0],
            'B' => [1, 0, 1, 1],
            'C' => [1, 1, 0, 0],
            'D' => [1, 1, 0, 1],
            'E' => [1, 1, 1, 0],
            'F' => [1, 1, 1, 1],
            _ => panic!("invalid bits found {}", num),
        };

        packets.append(&mut packet.to_vec());
    }

    packets.reverse();

    packets
}

// Calculates the packet header value.
fn header(packets: &mut Vec<u8>) -> Option<i64> {
    decimal_value(packets, 3)
}

// Calculate the number of sub packets.
fn length(packets: &mut Vec<u8>) -> Option<i64> {
    if packets.pop()? == 0 {
        decimal_value(packets, 15)
    } else {
        decimal_value(packets, 11)
    }
}

// Calculates a packet value and whether it is the last one.
fn value(packets: &mut Vec<u8>) -> Option<i64> {
    let mut end = false;
    let mut value = Vec::<u8>::new();

    while !end {
        end = packets.pop()? == 0;
        for _ in 0..4 {
            value.push(packets.pop()?);
        }
    }

    let length = value.len();
    value.reverse();

    decimal_value(&mut value, length)
}

// Calculate the decimal value of a binary array.
fn decimal_value(binary: &mut Vec<u8>, len: usize) -> Option<i64> {
    let mut value = 0;

    for index in 1..=len {
        let digit = match binary.pop() {
            Some(val) => val,
            None => return None,
        };
        value += 2i64.pow((len - index) as u32) * (digit as i64);
    }

    Some(value)
}

// Generate a token from the available packets.
fn parse_packet(packets: &mut Vec<u8>) -> Option<Token> {
    let version = header(packets)?;
    let class = header(packets)?;

    if class == 4 {
        let val = value(packets)?;

        Some(Token {
            version: version,
            class: class,
            value: val,
        })
    } else {
        let length = length(packets)?;

        Some(Token {
            version: version,
            class: class,
            value: length,
        })
    }
}

// Generate all tokens from the packets.
fn parse_packets(packets: &mut Vec<u8>) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();

    while let Some(token) = parse_packet(packets) {
        tokens.push(token);
    }

    // Reverse the tokens for easier processing.
    tokens.reverse();

    tokens
}

// Pretty prints the tokens for visualization.
pub fn pretty_print(tokens: &[Token]) {
    println!("message:");
    for index in (0..tokens.len()).rev() {
        match tokens[index].class {
            0 => print!("+ "),
            1 => print!("* "),
            2 => print!("min "),
            3 => print!("max "),
            4 => print!("{} ", tokens[index].value),
            5 => print!("> "),
            6 => print!("< "),
            7 => print!("== "),
            _ => print!("!!! {} ", tokens[index].class),
        }
    }
    println!();
}
