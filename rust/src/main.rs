use std::fs;

fn main() {
    if let Ok(content) = fs::read_to_string("./inputs/01.txt") {
        println!("Solution for Day 1: {}", content);
    };
}
