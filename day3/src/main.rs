use std::collections::HashMap;

const PUZZLE: &str = include_str!("input.txt");

fn main() {
    let claims = PUZZLE.lines().collect::<Vec<&str>>();

    let mut cuts: HashMap<(i32, i32), i32> = HashMap::new();

    claims.iter().for_each(|claim| {
        // Format: [0:ID, 1:fromTop, 2:fromLeft, 3:width, 4:height]
        let params = claim
            .split(|c| c == '#' || c == ' ' || c == '@' || c == ':' || c == 'x' || c == ',')
            .flat_map(|e| e.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        for x in params[1]..params[1]+params[3] {
            for y in params[2]..params[2]+params[4] {
                *cuts.entry((x, y)).or_insert(0) += 1;
            }
        }
    });

    let overlap = cuts.values().filter(|v| **v > 1).count();
    println!("Overlap: {}", overlap);
}
