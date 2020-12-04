use std::{fs, str::FromStr};

#[derive(Debug, Default)]
struct Document {
    // Birth Year
    byr: Option<String>,
    // Issue Year
    iyr: Option<String>,
    // Expiration Year
    eyr: Option<String>,
    // Height
    hgt: Option<String>,
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
        [
            &self.byr, &self.iyr, &self.eyr, &self.hgt, &self.hcl, &self.ecl, &self.pid,
        ]
        .iter()
        .all(|&part| part.is_some())
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
                    "byr" => document.byr = Some(String::from_str(value).unwrap()),
                    "iyr" => document.iyr = Some(String::from_str(value).unwrap()),
                    "eyr" => document.eyr = Some(String::from_str(value).unwrap()),
                    "hgt" => document.hgt = Some(String::from_str(value).unwrap()),
                    "hcl" => document.hcl = Some(String::from_str(value).unwrap()),
                    "ecl" => document.ecl = Some(String::from_str(value).unwrap()),
                    "pid" => document.pid = Some(String::from_str(value).unwrap()),
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
    // println!("Part 2: {}", number_of_trees_on_slope(&map));
}
