use std::fs;

const TARGET: u32 = 2020;

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("File doesn't exists");

    let mut result_1: u32 = 0;
    let mut result_2: u32 = 0;

    let all_expenses: Vec<u32> = file_content.trim().lines().map(|s| s.parse().unwrap()).collect();

    for x in 0..all_expenses.len() {
        for y in 1..all_expenses.len() {
            if all_expenses[x] + all_expenses[y] == TARGET {
                result_1 = all_expenses[x] * all_expenses[y];
            }

            for z in 0..all_expenses.len() {
                if (all_expenses[x] + all_expenses[y] + all_expenses[z]) == TARGET {
                    result_2 = all_expenses[x] * all_expenses[y] * all_expenses[z];
                }
            }
        }
    }

    println!("Part 1: {}", result_1);
    println!("Part 2: {}", result_2);
}
