use std::io;

mod day1;

fn main() -> io::Result<()> {
    day1::part1()?;
    day1::part2()?;
    Ok(())
}
