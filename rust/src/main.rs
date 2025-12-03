use std::fs;
use std::io;

mod day1;
mod day2;

fn main() -> io::Result<()> {
    day1::part1()?;
    day1::part2()?;
    let day2_input = &fs::read_to_string("./inputs/02")?;
    println!("Solution for Day 2 Part 1: {}", day2::part1(day2_input));
    // day2::part2()?;
    Ok(())
}
