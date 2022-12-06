use std::fs;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

// Prints the image as ASCII B/W.
fn print_image(image: &Vec<Vec<u32>>) {
    println!("\n###\n");

    for row in image {
        for pixel in row {
            match pixel {
                0 => print!(" "),
                1 => print!("#"),
                2 => print!(" "),
                _ => print!("!"),
            }
        }

        print!("\n");
    }

    println!("\n###\n");
}

// Calculate the checksum of a Space Image Format transmission
fn checksum(image: &Vec<Vec<Vec<u32>>>) {
    let mut histogram: Vec<HashMap<u32, u32>> = Vec::new();

    for layer in image {
        let mut map: HashMap<u32, u32> = HashMap::new();

        for row in layer {
            for pixel in row {
                let mut count: u32 = 1;

                if let Some(num) = map.get(pixel) {
                    count += num;
                }

                map.insert(*pixel, count);
            }

        }

        histogram.push(map.clone());
    }

    let mut fewest_zeros: u32 = std::u32::MAX;
    let mut checksum: u64 = 0;

    let chksm = |data: HashMap<u32, u32>| -> u64 {
        let (mut ones, mut twos): (u32, u32) = (0, 0);
        if let Some(&count) = data.get(&(1)) {
            ones = count;
        }

        if let Some(&count) = data.get(&(2)) {
            twos = count;
        }

        return ones as u64 * twos as u64;
    };

    for layer in histogram {
        if let Some(&num) = layer.get(&(0)) {
            if num < fewest_zeros {
                fewest_zeros = num;
                checksum = chksm(layer);
            }
        }

        else {
            fewest_zeros = 0;
            checksum = chksm(layer);
        }
    }

    println!("checksum: {}", checksum);
}

// Parse the layers to determine the visible pixels.
fn flatten(layers: &Vec<Vec<Vec<u32>>>) -> Vec<Vec<u32>> {
    let mut image: Vec<Vec<u32>> = Vec::new();

    if layers.len() == 0 {
        return image;
    }

    for _ in 0 .. layers[0].len() {
        let mut row: Vec<u32> = Vec::new();

        for _ in 0 .. layers[0][0].len() {
            row.push(2);
        }

        image.push(row)
    }

    for layer in layers {
        for row in 0 .. layer.len() {
            for col in 0 .. layer[0].len() {
                if image[row][col] == 2 {
                    image[row][col] = layer[row][col];
                }
            }
        }
    }

    return image;
}

fn main() -> io::Result<()> {
    // Get image width and height from user input.
    let mut raw_input = String::new();
    let (mut width, mut height): (i64, i64) = (0, 0);

    print!("image width: ");
    io::stdout().flush()?;

    if let Ok(_) = io::stdin().read_line(&mut raw_input) {
        raw_input.pop();

        if let Ok(num) = raw_input.parse::<i64>() {
            width = num;
        }

        else {
            panic!("not an int");
        }
    }

    raw_input.clear();
    print!("image height: ");
    io::stdout().flush()?;

    if let Ok(_) = io::stdin().read_line(&mut raw_input) {
        raw_input.pop();

        if let Ok(num) = raw_input.parse::<i64>() {
            height = num;
        }

        else {
            panic!("not an int");
        }
    }

    // Read the input into a "matrix".
    let mut input: String = fs::read_to_string("input")?;
    input = input[0 .. input.len()-1].to_string();
    let mut total_layers: usize = input.len() / ((width * height) as usize);

    if input.len() % (width * height) as usize != 0 {
        total_layers += 1;
    }

    let mut layers: Vec<Vec<Vec<u32>>> = Vec::new();
    for _ in 0 .. total_layers {
        let mut image: Vec<Vec<u32>> = Vec::new();

        for _ in 0 .. height {
            let mut col: Vec<u32> = Vec::new();
            col.resize(width as usize, 0);

            image.push(col);
        }

        layers.push(image);
    }

    let (mut row, mut col, mut layer): (usize, usize, usize) = (0, 0, 0);

    for pixel in input.chars() {
        layers[layer][row][col] = pixel.to_digit(10).unwrap();

        col += 1;

        if col >= width as usize {
            col = 0;
            row += 1;
        }

        if row >= height as usize {
            row = 0;
            layer += 1;
        }

    }

    checksum(&layers);
    print_image(&flatten(&layers));

    Ok(())
}
