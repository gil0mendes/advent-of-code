use std::{collections::HashMap, fs};

fn compute_part1(raw_data: &String) -> i32 {
    let mut repetitions = HashMap::new();
    let mut v_help = Vec::new();

    let mut twos = 0;
    let mut threes = 0;

    raw_data
        .lines()
        // Iterate all lines
        .for_each(|line| {
            // Fill the hash with the number of occurrences of each char
            line.chars().for_each(|c| {
                *repetitions.entry(c).or_insert(0) += 1;
            });

            // Drain the Hash into a vector and remove the ones that aren't twos or threes
            v_help.extend(repetitions.drain().map(|(_, v)| v));
            v_help.retain(|v| *v == 2 || *v == 3);

            // Followings the instructions we only accept one of each
            // type, so we remove the duplications
            v_help.dedup();

            // Update the number of twos and threes
            v_help.drain(..).for_each(|n| {
                if n == 2 {
                    twos += 1;
                } else {
                    threes += 1;
                }
            });
        });

    twos * threes
}

fn compute_part2(raw_data: &String) -> String {
    let boxes = raw_data.lines().collect::<Vec<&str>>();
    let mut common_chars = String::new();

    // Iterate all boxes
    'iter1: for (index, id1) in boxes.iter().enumerate() {
        // Iterate all subsequent boxes
        for id2 in boxes[index..].iter() {
            let mut num_differences = 0;

            // Join the two iterators into one to make
            // a side by side compaction
            for (char1, char2) in id1.chars().zip(id2.chars()) {
                if char1 != char2 {
                    num_differences += 1;
                } else {
                    common_chars.push(char1);
                }

                // Following the challenge explanation there is
                // only one char different
                if num_differences > 1 {
                    common_chars.clear();
                    break;
                }
            }

            if num_differences == 1 {
                break 'iter1;
            }
        }
    }

    common_chars
}

fn main() {
    let raw_data = fs::read_to_string("2018/day2/input.txt").unwrap();

    println!("Result part 1: {}", compute_part1(&raw_data));
    println!("Result part 2: {}", compute_part2(&raw_data));
}

#[test]
fn test_part1() -> Result<(), ()> {
    const INPUT: &str = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
";
    assert_eq!(compute_part1(&INPUT.to_string()), 12);
    Ok(())
}

#[test]
fn test_part2() -> Result<(), ()> {
    const INPUT: &str = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
";
    assert_eq!(compute_part2(&INPUT.to_string()), "fgij");
    Ok(())
}
