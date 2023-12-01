use std::fs;

fn compute_part1(raw_data: String) -> u32 {
    let all_numbers_per_row: Vec<Vec<String>> = raw_data
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .filter(|ch| ch.is_numeric())
                .map(|ch| ch.to_string())
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
    0
}

fn main() {
    let raw_data = fs::read_to_string("input.txt").expect("input file does not exists");

    println!("Result part 1: {}", compute_part1(raw_data.clone()));
    println!("Result part 1: {}", compute_part2(raw_data));
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
