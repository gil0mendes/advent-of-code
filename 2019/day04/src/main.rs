// //! Part 1: 1764
// //! Part 2: 1196

#![feature(drain_filter)]

fn is_increasing(chars: &Vec<u32>) -> bool {
    for i in 1..chars.len() {
        if chars[i] < chars[i-1] {
            return false
        }
    }

    true
}

fn has_grouping(chars: &Vec<u32>) -> bool {
    for i in 1..chars.len() {
        if chars[i] == chars[i-1] {
            return true;
        }
    }

    false
}

fn meet_criteria_part1<'r>(number: &'r mut String) -> bool {
    let chars: Vec<u32> = number.chars().map(|s| s.to_digit(10).unwrap()).collect();

    let result = is_increasing(&chars);
    if !result {
        return false;
    }

    has_grouping(&chars)
}

fn has_grouping_more_than_two(chars: &Vec<u32>) -> bool {
    let mut counts: Vec<u64> = vec![0; 10];

    for c in chars {
        counts[*c as usize] += 1;
    }

    for c in counts {
        if c == 2 {
            return true;
        }
    }

    false
}

fn meet_criteria_part2<'r>(number: &'r mut String) -> bool {
    let chars: Vec<u32> = number.chars().map(|s| s.to_digit(10).unwrap()).collect();

    let result = is_increasing(&chars);
    if !result {
        return false;
    }

    has_grouping_more_than_two(&chars)
}

fn main() {
    let range: Vec<String> = (152085..=670283).map(|v| v.to_string()).collect();
    
    let number_of_matches = range.clone()
        .drain_filter(meet_criteria_part1)
        .collect::<Vec<String>>()
        .len();

    println!("Part 1: {}", number_of_matches);

    let number_of_matches = range.clone()
        .drain_filter(meet_criteria_part2)
        .collect::<Vec<String>>()
        .len();

    println!("Part 2: {}", number_of_matches);
}
