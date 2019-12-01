use std::str::FromStr;
use std::error::Error;
use std::collections::{HashSet, HashMap};

const DATA: &str = include_str!("./input.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    /// Compute the manhattan distance between this and
    /// the given point.
    pub fn man_distance(&self, cord: Coordinate) -> i32 {
        (self.x - cord.x).abs() + (self.y - cord.y).abs()
    }

    /// Create a new iterator with all coordinates which the distance matches
    /// the given step.
    fn radar(self, step: i32) -> impl Iterator<Item=Coordinate> {
        (self.x - step..=self.x + step)
            .flat_map(move |x| {
                (self.y - step..=self.y + step)
                    .map(move |y| Coordinate { x, y })
            })
            .filter(move |&cord| self.man_distance(cord) == step)
    }
}

struct Map {
    // All coordinates present on the input
    locations: Vec<Coordinate>,
    // Known finite coordinates
    finite_locations: HashSet<Coordinate>,
    grid: HashMap<Coordinate, Coordinate>,
}

impl Map {
    pub fn new(locations: Vec<Coordinate>) -> Self {
        Self {
            finite_locations: HashSet::new(),
            locations,
            grid: HashMap::new(),
        }
    }

    /// Search for all finite locations.
    fn search_for_finite_location(&mut self) {
        for step in 0..100 {
            for location in &self.locations {
                if self.finite_locations.contains(&location) {
                    continue;
                }

                for cord in location.radar(step) {
                    let closest = match self.closest_location(cord) {
                        None => continue,
                        Some(closest) => closest,
                    };

                    self.grid.insert(cord, closest);
                }
            }

            for &cord in &self.locations {
                if !cord.radar(step).any(|c| self.grid.get(&c) == Some(&cord)) {
                    self.finite_locations.insert(cord);
                }
            }
        }
    }

    fn closest_location(&self, cord: Coordinate) -> Option<Coordinate> {
        let (mut min, mut unique) = (self.locations[0], true);

        for &location in &self.locations[1..] {
            if location.man_distance(cord) == min.man_distance(cord) {
                unique = false
            } else if location.man_distance(cord) < min.man_distance(cord) {
                min = location;
                unique = true;
            }
        }

        if !unique {
            None
        } else {
            Some(min)
        }
    }

    fn distance_sum(&self, cord: Coordinate) -> i32 {
        self.locations
            .iter()
            .map(|&loc| loc.man_distance(cord))
            .sum()
    }
}

fn part1(map: &Map) {
    let mut biggest_area = 0;
    for &cord in &map.finite_locations {
        let mut possible = 0;
        for &cord2 in map.grid.values() {
            if cord == cord2 {
                possible += 1;
            }
        }

        if possible > biggest_area {
            biggest_area = possible;
        }
    }

    println!("Part1: {}", biggest_area);
}

fn part2(map: &Map) {
    let range = 400;
    let mut size = 0;

    for x in -range..=range {
        for y in -range..=range {
            if map.distance_sum(Coordinate { x, y }) < 10_000 {
                size += 1;
            }
        }
    }

    println!("Part2: {}", size);
}

fn main() {
    // Convert all points into coordinates
    let coordinates: Vec<Coordinate> = DATA
        .lines()
        .map(|line| {
            let parts = line
                .split(", ")
                .flat_map(|v| v.parse::<i32>().ok())
                .collect::<Vec<i32>>();
            Coordinate {
                x: parts[0],
                y: parts[1],
            }
        })
        .collect::<Vec<Coordinate>>();

    let mut map = Map::new(coordinates);
    map.search_for_finite_location();

    part1(&map);
    part2(&map);
}
