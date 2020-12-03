#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fs;
use std::str::FromStr;

use regex::Regex;

struct Entry {
    min: usize,
    max: usize,
    char: String,
    password: String,
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 1-4 m: mrfmmbjxr
        lazy_static! {
            static ref ENTRY_REGEXP: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)$").unwrap();
        }

        let groups = ENTRY_REGEXP.captures(s).unwrap();

        Ok(Self {
            min: groups.get(1).unwrap().as_str().parse().unwrap(),
            max: groups.get(2).unwrap().as_str().parse().unwrap(),
            char: groups.get(3).unwrap().as_str().parse().unwrap(),
            password: groups.get(4).unwrap().as_str().parse().unwrap(),
        })
    }
}

fn is_valid_password(entry: &Entry) -> bool {
    let number_of_chars = entry
        .password
        .split("")
        .filter(|&c| c == entry.char)
        .count();
    number_of_chars >= entry.min && number_of_chars <= entry.max
}

fn number_of_valid_password(passwords: &Vec<Entry>) -> usize {
    passwords
        .into_iter()
        .filter(|e| is_valid_password(e))
        .collect::<Vec<&Entry>>()
        .len()
}

fn is_valid_password_2(entry: &Entry) -> bool {
    let chars = vec![
        entry.password.chars().nth(entry.min - 1).unwrap(),
        entry.password.chars().nth(entry.max - 1).unwrap(),
    ];

    chars[0] != chars[1]
        && (chars[0] == entry.char.chars().next().unwrap()
            || chars[1] == entry.char.chars().next().unwrap())
}

fn number_of_valid_password_2(passwords: &Vec<Entry>) -> usize {
    passwords
        .into_iter()
        .filter(|e| is_valid_password_2(e))
        .collect::<Vec<&Entry>>()
        .len()
}

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("File doesn't exists");

    let passwords: Vec<Entry> = file_content
        .trim()
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Part 1: {}", number_of_valid_password(&passwords));
    println!("Part 2: {}", number_of_valid_password_2(&passwords));
}
