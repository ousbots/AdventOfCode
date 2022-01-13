use day16::calculator::*;
use day16::parser::*;

fn main() {
    let mut tokens = parse_file("assets/input.txt");

    let version_total: i64 = tokens.iter().map(|token| token.version).sum();
    println!("version total {}", version_total);

    pretty_print(&tokens);

    let total = calculate(&mut tokens).unwrap();
    println!("message total {}", total);
}
