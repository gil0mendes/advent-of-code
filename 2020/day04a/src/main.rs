#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::{fs, str::FromStr};

use regex::Regex;
use units::HeightUnit;

mod units;

#[derive(Default)]
struct Document {
    // Birth Year
    byr: Option<u32>,
    // Issue Year
    iyr: Option<u32>,
    // Expiration Year
    eyr: Option<u32>,
    // Height
    hgt: Option<HeightUnit>,
    // Hair Color
    hcl: Option<String>,
    // Eye Color
    ecl: Option<String>,
    // Passport ID
    pid: Option<String>,
    // Country ID
    cid: Option<String>,
}

impl Document {
    fn is_valid(&self) -> bool {
        if [&self.byr, &self.iyr, &self.eyr]
            .iter()
            .all(|part| part.is_some())
            == false
        {
            return false;
        }

        if self.hgt.is_none() {
            return false;
        }

        [&self.hcl, &self.ecl, &self.pid]
            .iter()
            .all(|part| part.is_some())
    }

    fn has_valid_values(&self) -> bool {
        // Birth Year
        match self.byr {
            Some(year) if year < 1920 || year > 2002 => return false,
            None => return false,
            _ => {}
        };

        // Issue Year
        match self.iyr {
            Some(year) if year < 2010 || year > 2020 => return false,
            None => return false,
            _ => {}
        };

        // Expiration Year
        match self.eyr {
            Some(year) if year < 2020 || year > 2030 => return false,
            None => return false,
            _ => {}
        };

        // Height
        match self.hgt {
            Some(HeightUnit::Centimeters(value)) if (value < 150 || value > 193) => return false,
            Some(HeightUnit::Inches(value)) if (value < 59 || value > 76) => return false,
            None => return false,
            _ => {}
        }

        if self.hcl.is_none() {
            return false;
        }
        if self.ecl.is_none() {
            return false;
        }
        if self.pid.is_none() {
            return false;
        }

        true
    }
}

impl FromStr for Document {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut document = Document::default();

        s.split(" ")
            .map(|e| {
                let mut parts = e.split(":");
                (parts.next().unwrap(), parts.next().unwrap())
            })
            .for_each(|(key, value)| {
                match key {
                    "byr" => document.byr = Some(String::from_str(value).unwrap().parse().unwrap()),
                    "iyr" => document.iyr = Some(String::from_str(value).unwrap().parse().unwrap()),
                    "eyr" => document.eyr = Some(String::from_str(value).unwrap().parse().unwrap()),
                    "hgt" => document.hgt = HeightUnit::from_str(value).ok(),
                    "hcl" => {
                        lazy_static! {
                            static ref HAIR_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                        };

                        document.hcl = if HAIR_REGEX.is_match(value) {
                            Some(String::from_str(value).unwrap())
                        } else {
                            None
                        }
                    }
                    "ecl" => {
                        let valid_values = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                        document.ecl = if valid_values.contains(&value) {
                            Some(String::from_str(value).unwrap())
                        } else {
                            None
                        }
                    }
                    "pid" => {
                        lazy_static! {
                            static ref PID_REGEX: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
                        };

                        document.pid = if PID_REGEX.is_match(value) {
                            Some(String::from_str(value).unwrap())
                        } else {
                            None
                        }
                    }
                    "cid" => document.cid = Some(String::from_str(value).unwrap()),
                    _ => panic!("invalid property"),
                };
            });

        Ok(document)
    }
}

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("File doesn't exists");

    let documents: Vec<Document> = file_content
        .trim()
        // split by empty lines
        .split("\n\n")
        // make all documents have just one line and convert it into a Document
        .map(|e| e.replace("\n", " ").parse::<Document>().unwrap())
        .collect();

    // Part 1
    let len_valid_docs = documents.iter().filter(|&doc| doc.is_valid()).count();
    println!("Part 1: {:?}", len_valid_docs);

    // Part 2
    let len_valid_docs_pt2 = documents
        .iter()
        .filter(|&doc| doc.has_valid_values())
        .count();
    println!("Part 2: {:?}", len_valid_docs_pt2);
}
