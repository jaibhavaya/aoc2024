mod day1;

use std::fs;

use day1::solve;
use day1::solve2;

fn main() {
    println!("Advent of code 2024");
    let day1input = fs::read_to_string("inputs/day1").expect("Could not read day1 file");
    let day1result1 = solve(&day1input);
    println!("Day 1 first result: {}", day1result1);

    let day1result2 = solve2(&day1input);
    println!("Day 1 first result: {}", day1result2);
}
