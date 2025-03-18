mod day1;
mod day2;

use std::fs;

fn main() {
    println!("Advent of code 2024");

    // Day 1
    let day1input = fs::read_to_string("inputs/day1").expect("Could not read day1 file");

    let day1result1 = day1::solve(&day1input);
    println!("Day 1 first result: {}", day1result1);

    let day1result2 = day1::solve2(&day1input);
    println!("Day 1 first result: {}", day1result2);

    // Day2
    let day2input = fs::read_to_string("inputs/day2").expect("Could not read day2 file");

    let day2result1 = day2::solve(&day2input);
    println!("Day 2 first result: {}", day2result1);
}
