use itertools::Itertools;

use std::fs;

fn compute_part1(raw_data: String) -> u32 {
    let all_numbers_per_row: Vec<Vec<String>> = raw_data
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|ch| ch.is_numeric().then(|| ch.to_string()))
                .collect()
        })
        .collect();

    all_numbers_per_row
        .iter()
        .map(|v| {
            let number = format!("{}{}", v.first().unwrap(), v.last().unwrap());
            number.parse::<u32>().unwrap()
        })
        .sum()
}

fn compute_part2(raw_data: String) -> u32 {
    let valid_digits = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    raw_data
        .lines()
        .map(|line| {
            let (_, first) = valid_digits
                .iter()
                .enumerate()
                .filter_map(|(idx, digit)| line.find(digit).map(|pos| (pos, idx)))
                .min()
                .expect("no first digit found");

            let (_, last) = valid_digits
                .iter()
                .enumerate()
                .filter_map(|(idx, digit)| line.rfind(digit).map(|pos| (pos, idx)))
                .max()
                .expect("no last digit found");

            Ok::<(u32, u32), ()>((first as u32 % 9 + 1, last as u32 % 9 + 1))
        })
        .process_results(|it| it.map(|(d1, d2)| d1 * 10 + d2).sum::<u32>())
        .unwrap()
}

fn main() {
    let raw_data = fs::read_to_string("2023/day01/input.txt").expect("input file does not exists");

    println!("Result part 1: {}", compute_part1(raw_data.clone()));
    println!("Result part 2: {}", compute_part2(raw_data));
}

#[test]
fn test_part1() -> Result<(), ()> {
    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    assert_eq!(compute_part1(INPUT.to_string()), 142);
    Ok(())
}

#[test]
fn test_part2() -> Result<(), ()> {
    const INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    assert_eq!(compute_part2(INPUT.to_string()), 281);
    Ok(())
}
