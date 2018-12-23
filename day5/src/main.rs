const DATA: &str = include_str!("./input.txt");
const REACT_DISTANCE: u8 = 32;

fn main() {
    let polymer: Vec<u8> = DATA.as_bytes().to_vec();
    let mut reacted_polymer: Vec<u8> = Vec::new();

    let length = polymer.len();

    for index in 0..length {
        if reacted_polymer.len() > 0 && react(polymer[index], reacted_polymer[reacted_polymer.len() - 1]) {
            reacted_polymer.pop();
        } else {
            reacted_polymer.push(polymer[index]);
        }
    }

    let resulted_polymer = String::from_utf8(reacted_polymer).unwrap();
    println!("Part 1: {}", resulted_polymer .len());
}

/// Check if the two monomers reacts with each other
fn react(monomer1: u8, monomer2: u8) -> bool {
    if monomer1 < monomer2 {
        monomer2 - monomer1 == REACT_DISTANCE
    } else {
        monomer1 - monomer2 == REACT_DISTANCE
    }
}