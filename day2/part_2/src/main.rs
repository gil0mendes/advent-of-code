const PUZZLE: &str = include_str!("input.txt");

fn main() {
    let boxes = PUZZLE.lines().collect::<Vec<&str>>();
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

    print!("Common id: {}", common_chars);
}
