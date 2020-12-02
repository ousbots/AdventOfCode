use std::fs;

const TARGET_VALUE: i32 = 2020;

fn main() {
    let path = std::env::args().nth(1).expect("path argument missing");
    let file_content = fs::read_to_string(path).expect("failed to read file");
    let lines = file_content.lines();

    let mut nums = Vec::new();
    for line in lines {
        nums.push(line.parse::<i32>().expect("failed to convert to int32"))
    }

    for index in 0..nums.len() {
        if nums[index] > TARGET_VALUE {
            continue;
        }

        for second_index in (index + 1)..nums.len() {
            let first = nums[index];
            let second = nums[second_index];
            if first + second == TARGET_VALUE {
                println!("first: {}, second: {}, mult: {}", first, second, first * second);
                break;
            }
        }

        for second_index in (index + 1)..nums.len() {
            for third_index in (second_index + 1)..nums.len() {
                let first = nums[index];
                let second = nums[second_index];
                let third = nums[third_index];

                if first + second + third == TARGET_VALUE {
                    println!("first: {}, second: {}, third: {}, mult: {}", first, second, third, first * second * third);
                    return;
                }
            }
        }
    }
}
