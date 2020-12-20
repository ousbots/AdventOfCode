use std::fs;

#[derive(Debug)]
struct Passport {
    birth: String,   // byr
    issued: String,  // iyr
    expires: String, // eyr
    height: String,  // hgt
    hair: String,    // hcl
    eye: String,     // ecl
    id: String,      // pid
    country: String, // cid
}

// Loads passports from the given file and returns them as a vector.
fn load_passports(path: String) -> Vec<Passport> {
    let file_content = fs::read_to_string(path).expect("failed to read file");
    let groups = file_content.split("\n\n");

    let mut passports: Vec<Passport> = Vec::new();

    for group in groups {
        let mut passport = Passport {
            birth: String::from(""),
            issued: String::from(""),
            expires: String::from(""),
            height: String::from(""),
            hair: String::from(""),
            eye: String::from(""),
            id: String::from(""),
            country: String::from(""),
        };

        for data in group.split(&[' ', '\n'][..]) {
            let tokens: Vec<&str> = data.split(':').collect();
            match tokens[0].clone() {
                "byr" => passport.birth = tokens[1].to_string(),
                "iyr" => passport.issued = tokens[1].to_string(),
                "eyr" => passport.expires = tokens[1].to_string(),
                "hgt" => passport.height = tokens[1].to_string(),
                "hcl" => passport.hair = tokens[1].to_string(),
                "ecl" => passport.eye = tokens[1].to_string(),
                "pid" => passport.id = tokens[1].to_string(),
                "cid" => passport.country = tokens[1].to_string(),
                _ => println!("unknown token {:#?}", tokens),
            }
        }

        passports.push(passport);
    }

    passports
}

// Returns the number of passports in the given vector that have been filled according to day 4
// part 1 rules.
fn filled(passports: &Vec<Passport>) -> i32 {
    let mut filled = 0;

    for passport in passports {
        if passport.birth == "" {
            continue;
        }
        if passport.issued == "" {
            continue;
        }
        if passport.expires == "" {
            continue;
        }
        if passport.height == "" {
            continue;
        }
        if passport.hair == "" {
            continue;
        }
        if passport.eye == "" {
            continue;
        }
        if passport.id == "" {
            continue;
        }

        filled += 1;
    }

    filled
}

// Returns the number of passports in the given vector are valid according to the day 4 part 2
// rules.
fn validate(passports: &Vec<Passport>) -> i32 {
    let mut valid = 0;

    for passport in passports {
        // Check birth date.
        let birth: i32 = match passport.birth.parse() {
            Ok(v) => v,
            _ => 0,
        };
        if birth > 2002 || birth < 1920 {
            continue;
        }

        // Check issued date.
        let issued: i32 = match passport.issued.parse() {
            Ok(v) => v,
            _ => 0,
        };
        if issued > 2020 || issued < 2010 {
            continue;
        }

        // Check expiration date.
        let expires: i32 = match passport.expires.parse() {
            Ok(v) => v,
            _ => 0,
        };
        if expires > 2030 || expires < 2020 {
            continue;
        }

        // Check the height.
        let height_chars: Vec<char> = passport.height.chars().collect();
        if height_chars.len() < 3 {
            continue;
        }

        let units: String = height_chars[(height_chars.len() - 2)..height_chars.len()]
            .into_iter()
            .collect();
        let height: i32 = match height_chars[0..height_chars.len() - 2]
            .into_iter()
            .collect::<String>()
            .parse()
        {
            Ok(v) => v,
            _ => continue,
        };

        let metric = match units.as_str() {
            "cm" => true,
            "in" => false,
            _ => continue,
        };

        if metric {
            if height > 193 || height < 150 {
                continue;
            }
        } else {
            if height > 76 || height < 59 {
                continue;
            }
        }

        // Check hair color.
        let hair_chars: Vec<char> = passport.hair.chars().collect();
        if hair_chars.len() != 7 {
            continue;
        }
        if hair_chars[0] != '#' {
            continue;
        }

        let mut valid_color = true;
        for elem in hair_chars[1..hair_chars.len()].iter() {
            match elem {
                '0' => (),
                '1' => (),
                '2' => (),
                '3' => (),
                '4' => (),
                '5' => (),
                '6' => (),
                '7' => (),
                '8' => (),
                '9' => (),
                'a' => (),
                'b' => (),
                'c' => (),
                'd' => (),
                'e' => (),
                'f' => (),
                _ => valid_color = false,
            }
        }

        if !valid_color {
            continue;
        }

        // Check eye color.
        match passport.eye.as_str() {
            "amb" => (),
            "blu" => (),
            "brn" => (),
            "grn" => (),
            "gry" => (),
            "hzl" => (),
            "oth" => (),
            _ => continue,
        }

        // Check ID.
        if passport.id.len() != 9 {
            continue;
        }

        let _: i32 = match passport.id.parse() {
            Ok(v) => v,
            _ => continue,
        };

        valid += 1;
    }

    valid
}

fn main() {
    let path = std::env::args().nth(1).expect("path argument missing");
    let passports = load_passports(path);
    let filled_count = filled(&passports);
    let valid_count = validate(&passports);
    println!("{} filled passports", filled_count);
    println!("{} valid passports", valid_count);
}
