use std::fs;

// Check if a password contains between min and max number of required letters in the given
// password.
fn valid_password_sled(password: &str, required: char, min: i32, max: i32) -> bool {
    let mut required_count = 0;

    for elem in password.chars() {
        if elem == required {
            required_count += 1;
        }
    }

    if required_count >= min && required_count <= max {
        return true;
    }

    return false;
}

// Check if the given password has the required letter in exactly one of the two given positions.
fn valid_password_toboggan(password: &str, required: char, first: usize, second: usize) -> bool {
    let mut count = 0;
    let letters = password.chars().collect::<Vec<char>>();

    if first > letters.len() || second > letters.len() {
        return false;
    }

    if letters[first-1] == required {
        count += 1;
    }

    if letters[second-1] == required {
        count += 1;
    }

    if count == 1 {
        return true;
    }

    return false;
}

// Reads the file and determines the number of incorrect passwords.
fn main() {
    let path = std::env::args().nth(1).expect("path argument missing");
    let file_content = fs::read_to_string(path).expect("failed to read file");
    let lines = file_content.lines();

    let mut valid_sled_passwords = 0;
    let mut valid_toboggan_passwords = 0;
    for line in lines {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();

        if tokens.len() != 3 {
            println!("bad line found: {:#?}", tokens);
            continue;
        }

        let limits = tokens[0].split("-").collect::<Vec<&str>>();
        let min = limits[0].parse::<i32>().expect("failed to parse int");
        let max = limits[1].parse::<i32>().expect("failed to parse int");

        let required = tokens[1].split(":").collect::<Vec<&str>>()[0].chars().collect::<Vec<char>>()[0];
        let password = tokens[2];

        if valid_password_sled(password, required, min, max) {
            valid_sled_passwords += 1;
        }

        if valid_password_toboggan(password, required, min as usize, max as usize) {
            valid_toboggan_passwords += 1;
        }
    }

    println!("{} valid sled passwords found", valid_sled_passwords);
    println!("{} valid toboggan passwords found", valid_toboggan_passwords);
}
