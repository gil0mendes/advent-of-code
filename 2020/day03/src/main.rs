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

fn number_of_trees(map: &Vec<Vec<Tile>>) -> u32 {
    let mut col = 0;

    map.iter()
        .skip(1)
        .map(|line| {
            col = (col + 3) % line.len();
            match line.get(col).unwrap() {
                Tile::Open => 0,
                Tile::Tree => 1,
            }
        })
        .sum()
}

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("File doesn't exists");

    let map: Vec<Vec<Tile>> = file_content
        .trim()
        .lines()
        .map(|line| line.chars().map(chart_to_tile).collect())
        .collect();

    println!("Part 1: {}", number_of_trees(&map));
}
