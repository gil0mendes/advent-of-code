use std::fs;

/// Defines the operation mode
/// 
/// Position: the operands values represents memory positions
/// Immediate: the operands values represents the value itself
#[derive(Debug)]
enum OperationMode {
    Position,
    Immediate,
}

struct Machine {
    pc: usize,
    memory: Vec<i32>,

    input: Vec<i32>,
    output: Vec<i32>,
}

impl Machine {
    pub fn new(memory: Vec<i32>, input: Vec<i32>) -> Self {
        Self {
            pc: 0,
            memory,
            input,
            output: Vec::new(),
        }
    }

    /// Get the current opcode under execution
    fn get_opcode(&self) -> i32 {
        self.memory[self.pc] % 100
    }

    /// Get the mode for the given parameter
    fn get_param_mode(&self, offset: usize) -> OperationMode {
        match (self.memory[self.pc] as usize / (10_usize.pow(offset as u32 + 1))) % 10 {
            1 => OperationMode::Immediate,
            _ => OperationMode::Position
        }
    }

    /// Get values based on the operation mode
    fn get_param(&self, nth: usize) -> i32 {
        let memory = &self.memory;

        let param_mode = self.get_param_mode(nth);
        let offset = self.pc + nth;

        match param_mode {
            OperationMode::Position => {
                let address: usize = memory[offset] as usize;
                memory[address]
            },
            OperationMode::Immediate => {
                memory[offset]
            }
        }
    }

    /// Get address based on PC offset
    fn get_address(&self, nth: usize) -> usize {
        self.memory[self.pc + nth] as usize
    }

    pub fn execute_program(&mut self) {
        loop {
            let pc = self.pc;
            let opcode = self.get_opcode();

            let step = match opcode {
                // sum
                1 => {
                    let (v1, v2) = (self.get_param(1), self.get_param(2));
                    let target_location = self.memory[pc + 3];
                    self.memory[target_location as usize] = v1 + v2;
    
                    4
                },
                // mul
                2 => {
                    let (v1, v2) = (self.get_param(1), self.get_param(2));
                    let target_location = self.memory[pc + 3];
                    self.memory[target_location as usize] = v1 * v2;
    
                    4
                },
                // Store
                3 => {
                    let address = self.get_address(1);
                    self.memory[address] = self.input.remove(0);
    
                    2
                },
                // Read
                4 => {
                    let value = self.get_param(1);
                    self.output.push(value);
    
                    2
                },
                // Jump if true
                5 => {
                    let value = self.get_param(1);
                    if value != 0 {
                        let address = self.get_param(2);
                        self.pc = address as usize;
                        0
                    } else {
                        3
                    }
                },
                // Jump if false
                6 => {
                    let value = self.get_param(1);
                    if value == 0 {
                        let address = self.get_param(2);
                        self.pc = address as usize;
                        0
                    } else {
                        3
                    }
                },
                // less than
                7 => {
                    let (v1, v2) = (self.get_param(1), self.get_param(2));
                    let address = self.get_address(3);
                    self.memory[address] = (v1 < v2) as i32;

                    4
                },
                8 => {
                    let (v1, v2) = (self.get_param(1), self.get_param(2));
                    let address = self.get_address(3);
                    self.memory[address] = (v1 == v2) as i32;

                    4
                },
                99 => { break; },
                _ => {
                    panic!("[{}:{}] Opcode not recognized", self.pc, opcode)
                },
            };
    
            self.pc += step;
        }
    }
}



fn main() {
    let file_content = fs::read_to_string("input.txt").expect("File doesn't exists");
    
    let initial_memory: Vec<i32> = file_content
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut machine = Machine::new(initial_memory.clone(), vec![1]);
    machine.execute_program();
    println!("Part 1: {:?}", machine.output);

    let mut machine = Machine::new(initial_memory.clone(), vec![5]);
    machine.execute_program();
    println!("Part 2: {:?}", machine.output);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_halt() {
        let mut machine = Machine::new(vec![99], vec![0]);
        machine.execute_program();

        assert_eq!(machine.pc, 0);
    }

    #[test]
    fn test_write_to_memory() {
        let mut machine = Machine::new(vec![3, 3, 99, 0], vec![30]);
        machine.execute_program();

        assert_eq!(30, machine.memory[3]);
    }

    #[test]
    fn test_write_to_output() {
        let mut machine = Machine::new(vec![4, 2, 99], vec![]);
        machine.execute_program();

        assert_eq!(99, machine.output[0]);
    }

    #[test]
    fn test_add() {
        let mut machine = Machine::new(vec![1, 2, 2, 0, 99], vec![]);
        machine.execute_program();

        assert_eq!(4, machine.memory[0]);
    }

    #[test]
    fn test_mul() {
        let mut machine = Machine::new(vec![2, 2, 4, 0, 99], vec![]);
        machine.execute_program();

        assert_eq!(396, machine.memory[0]);
    }

    #[test]
    fn test_immediate_mode() {
        let mut machine = Machine::new(vec![1102, 2, 4, 0, 99], vec![]);
        machine.execute_program();

        assert_eq!(8, machine.memory[0]);
    }

    #[test]
    fn test_position_mode_equal() {
        let mut machine = Machine::new(vec![3,9,8,9,10,9,4,9,99,-1,8], vec![8]);
        machine.execute_program();

        assert_eq!(1, machine.output[0]);
    }

    #[test]
    fn test_position_mode_not_equal() {
        let mut machine = Machine::new(vec![3,9,8,9,10,9,4,9,99,-1,8], vec![10]);
        machine.execute_program();

        assert_eq!(0, machine.output[0]);
    }
    
    #[test]
    fn test_position_mode_less_than() {
        let mut machine = Machine::new(vec![3,9,7,9,10,9,4,9,99,-1,8], vec![3]);
        machine.execute_program();

        assert_eq!(1, machine.output[0]);
    }

    #[test]
    fn test_position_mode_greater_than() {
        let mut machine = Machine::new(vec![3,9,7,9,10,9,4,9,99,-1,8], vec![10]);
        machine.execute_program();

        assert_eq!(0, machine.output[0]);
    }

    #[test]
    fn test_immediate_mode_equal() {
        let mut machine = Machine::new(vec![3,3,1108,-1,8,3,4,3,99], vec![8]);
        machine.execute_program();

        assert_eq!(1, machine.output[0]);
    }

    #[test]
    fn test_immediate_mode_not_equal() {
        let mut machine = Machine::new(vec![3,3,1108,-1,8,3,4,3,99], vec![10]);
        machine.execute_program();

        assert_eq!(0, machine.output[0]);
    }
    
    #[test]
    fn test_immediate_mode_less_than() {
        let mut machine = Machine::new(vec![3,3,1107,-1,8,3,4,3,99], vec![3]);
        machine.execute_program();

        assert_eq!(1, machine.output[0]);
    }

    #[test]
    fn test_immediate_mode_greater_than() {
        let mut machine = Machine::new(vec![3,3,1107,-1,8,3,4,3,99], vec![10]);
        machine.execute_program();

        assert_eq!(0, machine.output[0]);
    }

    #[test]
    fn test_position_jump_zero() {
        let mut machine = Machine::new(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], vec![0]);
        machine.execute_program();

        assert_eq!(0, machine.output[0]);
    }

    #[test]
    fn test_position_jump_one() {
        let mut machine = Machine::new(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], vec![100]);
        machine.execute_program();

        assert_eq!(1, machine.output[0]);
    }

    #[test]
    fn test_immediate_jump_zero() {
        let mut machine = Machine::new(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], vec![0]);
        machine.execute_program();

        assert_eq!(0, machine.output[0]);
    }

    #[test]
    fn test_immediate_jump_one() {
        let mut machine = Machine::new(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], vec![100]);
        machine.execute_program();

        assert_eq!(1, machine.output[0]);
    }
}
