use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let path = "assets/input.txt";

    let file = match File::open(path) {
        Ok(handle) => handle,
        Err(err) => {
            println!("invalid files {}: {}", path, err);
            return;
        }
    };

    let numbers: Vec<i32> = io::BufReader::new(file)
        .lines()
        .map(|x| match x {
            Ok(num) => num.parse::<i32>().unwrap(),
            Err(err) => {
                println!("invalid number: {}", err);
                -1
            }
        })
        .collect();

    if numbers.len() < 4 {
        println!("need more than {} numbers to compute", numbers.len());
        return;
    }

    let increases = count_increases(&numbers);
    println!("found {} depth increases", increases);

    let sliding_increases = sliding_window_increases(&numbers);
    println!("found {} sliding window depth increases", sliding_increases);
}

// Count the number of times that an element in the given vector is greater than the previous element.
fn count_increases(nums: &Vec<i32>) -> i32 {
    let mut count = 0;

    for index in 1..nums.len() {
        if nums[index] > nums[index - 1] {
            count += 1;
        }
    }

    count
}

// Count the number of times that the sum of a three element sliding window is greater than the sum
// of the previous sliding window.
fn sliding_window_increases(nums: &Vec<i32>) -> i32 {
    let mut count = 0;
    let mut window = nums[0] + nums[1] + nums[2];

    for index in 3..nums.len() {
        let new_window = window + nums[index] - nums[index - 3];

        if new_window > window {
            count += 1;
        }

        window = new_window;
    }

    count
}
