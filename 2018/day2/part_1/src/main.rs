const PUZZLE: &str = include_str!("input.txt");

use std::collections::HashMap;

fn main() {
    let mut repetitions = HashMap::new();
    let mut v_help = Vec::new();

    let mut twos = 0;
    let mut threes = 0;

    PUZZLE
        .lines()
        // Iterate all lines
        .for_each(|line| {
            // Fill the hash with the number of occurrences of
            // each char
            line.chars().for_each(|c| {
                *repetitions.entry(c).or_insert(0) += 1;
            });

            // Drain the Hash into a vector and remove the ones that
            // aren't twos or threes
            v_help.extend(repetitions.drain().map(|(_, v)| v));
            v_help.retain(|v| *v == 2 || *v == 3);

            // Followings the instructions we only accept one of each
            // type, so we remove the duplications
            v_help.dedup();

            // Update the number of twos and threes
            v_help.drain(..).for_each(|n| if n == 2 {
                twos += 1;
            } else {
                threes += 1;
            });
        });

    println!("Hash: {}", twos * threes);
}
