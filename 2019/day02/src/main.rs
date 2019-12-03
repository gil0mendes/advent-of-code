
use std::fs;

fn get_values(memory: &Vec<u32>, pc: usize) -> (u32, u32) {
    let (op1, op2) = (memory[pc + 1], memory[pc + 2]);
    (memory[op1 as usize], memory[op2 as usize])
}

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("File doesn't exists");
    
    let mut memory: Vec<u32> = file_content
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut pc: usize = 0;

    loop {
        let opcode = memory[pc];

        match opcode {
            // sum
            1 => {
                let (v1, v2) = get_values(&memory, pc);
                let target_location = memory[pc + 3];
                memory[target_location as usize] = v1 + v2;
            },
            // mul
            2 => {
                let (v1, v2) = get_values(&memory, pc);
                let target_location = memory[pc + 3];
                memory[target_location as usize] = v1 * v2;
            },
            99 => { break; },
            _ => { },
        }

        pc += 4;
    }

    println!("Answer: {}", memory[0]);
}
