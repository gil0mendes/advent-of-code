const DATA: &str = include_str!("./input.txt");
const REACT_DISTANCE: u8 = 32;

/// Check if the two monomers reacts with each other
fn react(monomer1: u8, monomer2: u8) -> bool {
    if monomer1 < monomer2 {
        monomer2 - monomer1 == REACT_DISTANCE
    } else {
        monomer1 - monomer2 == REACT_DISTANCE
    }
}

/// Apply a reduce operation to a Polymer
fn reduce(polymer: &str) -> String {
    let polymer = polymer.as_bytes().to_vec();
    let mut reacted_polymer: Vec<u8> = Vec::new();

    let length = polymer.len();

    for index in 0..length {
        if reacted_polymer.len() > 0 && react(polymer[index], reacted_polymer[reacted_polymer.len() - 1]) {
            reacted_polymer.pop();
        } else {
            reacted_polymer.push(polymer[index]);
        }
    }

    String::from_utf8(reacted_polymer).unwrap()
}

/// Improve polymer by checking which type of monomers
/// types must be removed.
fn improve_polymer(polymer: &str) -> usize {
    let mut shorter = polymer.len();

    for c in b'a'..b'z' {
        let char_to_remove = c as char;
        let upper_char = char_to_remove.to_ascii_uppercase();
        let new_polymer = polymer.replace(char_to_remove, "").replace(upper_char, "");
        let length = reduce(&new_polymer).len();

        if length < shorter {
            shorter = length;
        }
    }

    shorter
}

fn main() {
    println!("Part 1: {}", reduce(DATA).len());
    println!("Part 2: {}", improve_polymer(DATA));
}