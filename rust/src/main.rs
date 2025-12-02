use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    // Day 1
    let file = File::open("./inputs/01.txt")?;
    let file = BufReader::new(file);

    // Part 1
    let mut dial = 50;
    let mut password = 0;
    for line in file.lines() {
        let line = line?;
        let direction = &line[..1];
        let magnitude = line[1..].parse::<i32>().expect("Parse integer error");
        if direction == "L" {
            dial -= magnitude;
        } else if direction == "R" {
            dial += magnitude;
        } else {
            panic!();
        }
        dial %= 100;
        if dial < 0 {
            dial += 100;
        }
        if dial == 0 {
            password += 1;
        }
    }
    println!("Solution for Day 1 Part 1: {password}");

    Ok(())
}
