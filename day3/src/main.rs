use std::collections::{HashMap, HashSet};

const PUZZLE: &str = include_str!("input.txt");

fn main() {
    let claims = PUZZLE.lines().collect::<Vec<&str>>();

    let mut cuts: HashMap<(i32, i32), i32> = HashMap::new();

    let mut overlap_ids: HashMap<(i32, i32), i32> = HashMap::new();

    let mut overlap_cuts = HashSet::new();
    let mut all_cuts = HashSet::new();

    claims.iter().for_each(|claim| {
        // Format: [0:ID, 1:fromTop, 2:fromLeft, 3:width, 4:height]
        let params = claim
            .split(|c| c == '#' || c == ' ' || c == '@' || c == ':' || c == 'x' || c == ',')
            .flat_map(|e| e.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        for x in params[1]..params[1]+params[3] {
            for y in params[2]..params[2]+params[4] {
                *cuts.entry((x, y)).or_insert(0) += 1;

                // Insert all cuts on a hash set and register all overlapped claims into
                // another set to perform a diff between the two at the end
                all_cuts.insert(params[0]);

                if overlap_ids.contains_key(&(x, y)) {
                    overlap_cuts.insert(overlap_ids[&(x, y)]);
                    overlap_cuts.insert(params[0]);
                } else {
                    overlap_ids.insert((x, y), params[0]);
                }
            }
        }
    });

    let overlap = cuts.values().filter(|v| **v > 1).count();
    println!("Overlap: {}", overlap);

    let non_overlapped: HashSet<_> = all_cuts.difference(&overlap_cuts).collect();
    println!("Non Overlap: {:?}", non_overlapped);
}
