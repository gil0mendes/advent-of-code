#![feature(iterator_fold_self)]

use std::fs;

#[derive(Debug)]
enum Tile {
    Open,
    Tree,
}

fn chart_to_tile(ch: char) -> Tile {
    match ch {
        '.' => Tile::Open,
        _ => Tile::Tree,
    }
}

fn get_number_of_trees(map: &Vec<Vec<Tile>>, (down, right): (u32, u32)) -> u32 {
    let mut x = right;
    let mut y = down;
    let mut number_of_trees: u32 = 0;

    loop {
        let line = map.get(y as usize).unwrap();

        number_of_trees += match line.get(x as usize).unwrap() {
            Tile::Open => 0,
            Tile::Tree => 1,
        };

        x = (x + right) % line.len() as u32;
        y += down;

        if y as usize >= map.len() {
            return number_of_trees;
        }
    }
}

fn number_of_trees_on_slope(map: &Vec<Vec<Tile>>) -> usize {
    let slopes_to_test = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    slopes_to_test
        .iter()
        .map(|&slope| get_number_of_trees(map, slope) as usize)
        .fold_first(|acc, r| acc * r)
        .unwrap()
}

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("File doesn't exists");

    let map: Vec<Vec<Tile>> = file_content
        .trim()
        .lines()
        .map(|line| line.chars().map(chart_to_tile).collect())
        .collect();

    println!("Part 1: {}", get_number_of_trees(&map, (1, 3)));
    println!("Part 2: {}", number_of_trees_on_slope(&map));
}
