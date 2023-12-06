#![feature(iter_advance_by)]

use std::{fs, str::Lines, vec};

use regex::Regex;

#[derive(Debug)]
struct MapEntry {
    source_interval: (u64, u64),
    destination_interval: (u64, u64),
}

impl MapEntry {
    pub fn new(source: u64, destination: u64, length: u64) -> Self {
        MapEntry {
            source_interval: (source, source + length - 1),
            destination_interval: (destination, destination + length - 1),
        }
    }

    pub fn get_destination(&self, source: u64) -> Option<u64> {
        if source >= self.source_interval.0 && source <= self.source_interval.1 {
            return Some(self.destination_interval.0 + (source - self.source_interval.0));
        }

        None
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: Vec<MapEntry>,
    soil_to_fertilizer: Vec<MapEntry>,
    fertilizer_to_water: Vec<MapEntry>,
    water_to_light: Vec<MapEntry>,
    light_to_temperature: Vec<MapEntry>,
    temperature_to_humidity: Vec<MapEntry>,
    humidity_to_location: Vec<MapEntry>,
}

impl Almanac {
    fn source_to_destination(&self, maps: &Vec<MapEntry>, source: u64) -> u64 {
        for entry in maps {
            let mapped = entry.get_destination(source);

            if mapped.is_some() {
                return mapped.unwrap();
            }
        }

        source
    }

    pub fn lowest_seed_number(&self) -> u64 {
        self.seeds
            .iter()
            .map(|&seed| {
                let soil = self.source_to_destination(&self.seed_to_soil, seed);
                let fertilizer = self.source_to_destination(&self.soil_to_fertilizer, soil);
                let water = self.source_to_destination(&self.fertilizer_to_water, fertilizer);
                let light = self.source_to_destination(&self.water_to_light, water);
                let temperature = self.source_to_destination(&self.light_to_temperature, light);
                let humidity =
                    self.source_to_destination(&self.temperature_to_humidity, temperature);

                self.source_to_destination(&self.humidity_to_location, humidity)
            })
            .min()
            .unwrap()
    }
}

fn parse_line_numbers(line: &str) -> Vec<u64> {
    let numbers_re = Regex::new(r"(\d+)").unwrap();

    numbers_re
        .find_iter(line)
        .filter_map(|digits| digits.as_str().parse().ok())
        .collect()
}

/// Parse the next section
fn parse_section(lines_iter: &mut Lines) -> Vec<MapEntry> {
    // ignore the first line, which contains the section name
    lines_iter.next();

    let mut map_entries = Vec::new();

    while let Some(line) = lines_iter.next() {
        // break when we find an empty line, this means that we reached to the end of the section
        if line.is_empty() {
            break;
        }

        // parse number and created a new entry
        let captures = parse_line_numbers(line);
        map_entries.push(MapEntry::new(captures[1], captures[0], captures[2]));
    }

    map_entries
}

fn parse(raw_data: &String) -> Almanac {
    let mut lines = raw_data.lines();

    // the input follows the same structure as the example, so at the top we have the seeds
    let seeds: Vec<u64> = parse_line_numbers(lines.next().unwrap());

    // skip empty line after the seeds section
    lines.next();

    // since all the sections follows the same order there is no need to create more dynamic code
    Almanac {
        seeds,
        seed_to_soil: parse_section(&mut lines),
        soil_to_fertilizer: parse_section(&mut lines),
        fertilizer_to_water: parse_section(&mut lines),
        water_to_light: parse_section(&mut lines),
        light_to_temperature: parse_section(&mut lines),
        temperature_to_humidity: parse_section(&mut lines),
        humidity_to_location: parse_section(&mut lines),
    }
}

fn compute_part1(raw_data: &String) -> u64 {
    let data = parse(raw_data);

    data.lowest_seed_number()
}

fn compute_part2(raw_data: &String) -> u64 {
    0
}

fn main() {
    let raw_data = fs::read_to_string("2023/day05/input.txt").expect("input file does not exists");

    println!("Result part 1: {}", compute_part1(&raw_data));
    println!("Result part 2: {}", compute_part2(&raw_data));
}

#[test]
fn test_part1() -> Result<(), ()> {
    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    assert_eq!(35, compute_part1(&INPUT.to_string()));
    Ok(())
}

#[test]
fn test_part2() -> Result<(), ()> {
    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    assert_eq!(30, compute_part2(&INPUT.to_string()));
    Ok(())
}
