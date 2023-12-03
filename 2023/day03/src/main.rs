use std::{collections::HashMap, fs};

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point(usize, usize);

impl Point {
    fn shift(&self, s: (i32, i32)) -> Point {
        Point(
            (self.0 as i32 + s.0) as usize,
            (self.1 as i32 + s.1) as usize,
        )
    }
}

type NumberIndex = usize;
#[derive(Debug)]
enum Entry {
    Symbol(char),
    Number(NumberIndex),
}

impl Entry {
    pub fn is_symbol(&self) -> bool {
        match self {
            Entry::Symbol(_) => true,
            _ => false,
        }
    }
}

fn parse_data(raw_data: &String) -> (HashMap<Point, Entry>, Vec<u32>) {
    let mut numbers = Vec::new();
    let mut data: HashMap<Point, Entry> = HashMap::new();

    raw_data.lines().enumerate().for_each(|(line_num, line)| {
        let mut prev_has_number = false;

        line.chars().enumerate().for_each(|(char_pos, ch)| {
            match ch {
                '.' => {
                    prev_has_number = false;
                }
                '0'..='9' => {
                    if prev_has_number {
                        let last_number = numbers
                            .last_mut()
                            .expect("if the last was a number something is odd!");
                        *last_number = *last_number * 10 + ch.to_digit(10).unwrap();
                    } else {
                        numbers.push(ch.to_digit(10).unwrap());
                    };

                    data.insert(Point(line_num, char_pos), Entry::Number(numbers.len() - 1));
                    prev_has_number = true;
                }
                symbol => {
                    data.insert(Point(line_num, char_pos), Entry::Symbol(symbol));
                    prev_has_number = false;
                }
            };
        })
    });

    (data, numbers)
}

fn point_is_symbol(data: &HashMap<Point, Entry>, point: Point) -> bool {
    data.get(&point).map_or(false, |e| e.is_symbol())
}

const neighbors_shits: [(i32, i32); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

fn compute_part1(raw_data: &String) -> u32 {
    let (data, numbers) = parse_data(raw_data);

    data.iter()
        .filter_map(|(point, entry)| match entry {
            Entry::Number(number_index) => neighbors_shits
                .iter()
                .any(|shift| point_is_symbol(&data, point.shift(*shift)))
                .then(|| number_index),
            _ => None,
        })
        .unique()
        .map(|&index| numbers.get(index).unwrap())
        .sum()
}

fn compute_part2(raw_data: &String) -> u32 {
    let (data, numbers) = parse_data(raw_data);

    // 1. get all the gears, by searching for '*'
    // 2. get all neighbors of each gear and if they have two numbers return the gear ratio
    // 3. sum all the products
    data.iter()
        .filter_map(|(point, entry)| match entry {
            Entry::Symbol('*') => {
                let neighbors = neighbors_shits
                    .iter()
                    .filter_map(|&shift| match data.get(&point.shift(shift)) {
                        Some(Entry::Number(number_index)) => Some(*number_index),
                        _ => None,
                    })
                    .unique()
                    .collect::<Vec<usize>>();

                if neighbors.len() > 1 {
                    Some(
                        neighbors
                            .iter()
                            .map(|&index| numbers[index])
                            .product::<u32>(),
                    )
                } else {
                    None
                }
            }
            _ => None,
        })
        .sum()
}

fn main() {
    let raw_data = fs::read_to_string("2023/day03/input.txt").expect("input file does not exists");

    println!("Result part 1: {}", compute_part1(&raw_data));
    println!("Result part 2: {}", compute_part2(&raw_data));
}

#[test]
fn test_part1() -> Result<(), ()> {
    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    assert_eq!(4361, compute_part1(&INPUT.to_string()));
    Ok(())
}

#[test]
fn test_part2() -> Result<(), ()> {
    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    assert_eq!(467835, compute_part2(&INPUT.to_string()));
    Ok(())
}
