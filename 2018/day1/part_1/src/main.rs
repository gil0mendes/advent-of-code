const PUZZLE: &str = include_str!("input.txt");

fn main() {
    let frequency: i32 = PUZZLE.lines().filter_map(|v| v.parse::<i32>().ok()).sum();
    println!("Final frequency {}", frequency);
}
