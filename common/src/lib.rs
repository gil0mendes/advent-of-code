use regex::Regex;

pub fn parse_line_numbers(line: &str) -> Vec<u64> {
    let numbers_re = Regex::new(r"(\d+)").unwrap();

    numbers_re
        .find_iter(line)
        .filter_map(|digits| digits.as_str().parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_numbers_on_a_str() {
        let result = parse_line_numbers("Time:      7  15   30");
        assert_eq!(result, [7, 15, 30]);
    }
}
