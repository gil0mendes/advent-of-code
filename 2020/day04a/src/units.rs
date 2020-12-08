use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
pub enum HeightParseError {
    InvalidFormat,
}

pub enum HeightUnit {
    Inches(u32),
    Centimeters(u32),
}

impl FromStr for HeightUnit {
    type Err = HeightParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref HEIGHT_REGEX: Regex =
                Regex::new(r"^(?P<value>\d+)(?P<unit>cm|in)$").unwrap();
        }

        if let Some(groups) = HEIGHT_REGEX.captures(s) {
            if let Ok(value) = usize::from_str(groups.name("value").unwrap().as_str()) {
                let new_value = match groups.name("unit") {
                    Some(x) if x.as_str() == "in" => HeightUnit::Inches(value as u32),
                    Some(x) if x.as_str() == "cm" => HeightUnit::Centimeters(value as u32),
                    _ => return Err(HeightParseError::InvalidFormat),
                };

                return Ok(new_value);
            }
        }

        Err(HeightParseError::InvalidFormat)
    }
}
