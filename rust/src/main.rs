use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn day1part1() -> io::Result<()> {
    let mut dial = 50;
    let mut password = 0;
    let file = File::open("./inputs/01.txt")?;
    let file = BufReader::new(file);
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

fn day1part2() -> io::Result<()> {
    let mut dial = 50;
    let mut password = 0;
    for line in BufReader::new(File::open("./inputs/01.txt")?).lines() {
        let line = line?;
        let clicks = line[1..].parse::<i32>().expect("Parse integer error")
            * if &line[..1] == "L" { -1 } else { 1 };
        println!("dial: {dial}, clicks: {clicks}");
        dial += clicks;
        match dial.cmp(&0) {
            Ordering::Less => {
                if dial != clicks {
                    password += 1;
                }
                if dial < -99 {
                    password -= dial / 100;
                    if dial % 100 == 0 {
                        dial = 0;
                    } else {
                        dial = 100 + (dial % 100);
                    }
                } else {
                    dial = 100 - dial.abs();
                }
            }
            Ordering::Equal => {
                password += 1;
            }
            Ordering::Greater => {
                if dial > 99 {
                    password += dial / 100;
                    dial %= 100;
                }
            }
        }
        println!("password: {password}");
    }
    println!("Solution for Day 1 Part 2: {password}");
    Ok(())
}

fn main() -> io::Result<()> {
    day1part1()?;
    day1part2()?;
    Ok(())
}
