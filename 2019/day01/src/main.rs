//! the values below are the right answers
//! Part 1: 3452245
//! Part 2: -

use std::{
    fs::File,
    io::{BufRead, BufReader}
};

fn get_input() -> Vec<u32> {
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|s| s.unwrap().parse::<u32>().unwrap())
        .collect()
}

fn call_fuel_for_mass(mass: &u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn calc_integral_fuel(mass: &u32) -> u32 {
    match call_fuel_for_mass(mass) {
        0 => 0,
        fuel => fuel + calc_integral_fuel(&fuel)
    }
}

fn main() {
    let modules = get_input();

    let part1: u32 = modules.iter().map(call_fuel_for_mass).sum();
    let part2: u32 = modules.iter().map(calc_integral_fuel).sum();

    

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}
