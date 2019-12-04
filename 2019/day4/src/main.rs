use std::io;

// Check if this is a valid elf password number
fn is_valid(mut num: i64) -> bool {
    // Build a vector of digits
    let mut digits = Vec::new();
    while num > 9 {
        digits.push(num % 10);
        num = num / 10;
    }
    digits.push(num);

    // Miniumum length is 6
    if digits.len() < 6 {
        return false;
    }

    let mut double = false;
    let mut prev_digit_count = 1;
    let mut prev_digit = digits[0];

    for digit in digits[1 .. ].into_iter() {
        // Check for doubled (not more) digits
        if *digit == prev_digit {
            prev_digit_count += 1;
        }

        else {
            if prev_digit_count == 2 {
                double = true;
            }

            prev_digit_count = 1;
        }

        // Check that digits never decrease (digits are reversed)
        if *digit > prev_digit {
            return false;
        }

        prev_digit = *digit;
    }

    // Check if the last digit was a double.
    if prev_digit_count == 2 {
        double = true;
    }

    return double;
}

fn main() -> io::Result<()> {
    let begin = 156218;
    let end = 652528;
    let mut count: i64 = 0;

    for number in begin .. end+1 {
        if is_valid(number) {
            count += 1;
        }
    }

    println!("########\n#tests:#\n########");
    println!("({}) [true]:\t{}", 112233, is_valid(112233));
    println!("({}) [false]:\t{}", 123444, is_valid(123444));
    println!("({}) [true]:\t{}\n\n", 111122, is_valid(111122));

    println!("possible passwords in range {} - {}: {}", begin, end, count);

    Ok(())
}
