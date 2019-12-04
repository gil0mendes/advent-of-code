use std::fs;
use std::collections::HashSet;

fn convert_inst_to_step(inst: &str) -> impl Iterator<Item = (isize, isize)> {
    let mut chars = inst.chars();

    let coord = match chars.next() {
        Some('U') => (0, -1),
        Some('D') => (0,  1),
        Some('L') => (-1, 0),
        Some('R') => (1, 0),
        _         => (0, 0)
    };

    let dist: usize  = chars.collect::<String>().parse().unwrap();
    std::iter::repeat(coord).take(dist)
}

fn parse_line(line: &str) -> Vec<(isize, isize)> {
    line.split(",").flat_map(convert_inst_to_step).scan((0, 0), |pos, step| {
        pos.0 += step.0;
        pos.1 += step.1;
        Some(pos.clone())
    }).collect()
}

fn compute_distance_to_center(pos: &(isize, isize)) -> usize {
    pos.0.abs() as usize + pos.1.abs() as usize
}

fn compute_coord_distance(coord: (isize, isize), pos1: &Vec<(isize, isize)>, pos2: &Vec<(isize, isize)>) -> usize {
    pos1.iter().position(|v| *v == coord).unwrap() + pos2.iter().position(|v| *v == coord).unwrap() + 2
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("The doesn't exists on that path");
    let mut lines = data.lines();

    // parse the two paths
    let position1 = parse_line(lines.next().unwrap());
    let position2 = parse_line(lines.next().unwrap());

    // intersect the two paths using the HashSet methods
    let position1_set: HashSet<_> = position1.iter().cloned().collect();
    let position2_set: HashSet<_> = position2.iter().cloned().collect();

    let collisions = position1_set.intersection(&position2_set);

    let closest_point = collisions.clone().map(compute_distance_to_center).min();
    println!("Part 1: {:?}", closest_point.unwrap());

    let quick_collision = collisions.map(|coord| compute_coord_distance(coord.clone(), &position1, &position2)).min();
    println!("Part 2: {:?}", quick_collision.unwrap());
}
