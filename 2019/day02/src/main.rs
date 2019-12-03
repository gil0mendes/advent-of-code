
use std::fs;

fn get_values(memory: &Vec<u32>, pc: usize) -> (u32, u32) {
    let (op1, op2) = (memory[pc + 1], memory[pc + 2]);
    (memory[op1 as usize], memory[op2 as usize])
}

fn execute_program(mut memory: Vec<u32>) -> Vec<u32> {
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

    memory
}

fn search_combination(init_memory: Vec<u32>, target: u32) -> u32 {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut memory = init_memory.clone();
            memory[1] = noun;
            memory[2] = verb;

            let result = execute_program(memory);
            if result[0] == target {
                return 100 * noun + verb;
            }
        }
    }

    0
}

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("File doesn't exists");
    
    let initial_memory: Vec<u32> = file_content
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Part 1 answer: {}", execute_program(initial_memory.clone())[0]);
    println!("Part 2 answer: {}", search_combination(initial_memory, 19690720));
    
}
