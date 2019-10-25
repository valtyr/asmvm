use crate::instruction::Opcode;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
    conditional: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
            conditional: false,
        }
    }

    pub fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        return result;
    }

    pub fn step(&mut self) {
        self.execute_instruction();
    }

    pub fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }

        match self.decode_opcode() {
            Opcode::HLT => {
                println!("HLT encountered");
                return false;
            }
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;
                self.registers[register] = number as i32;
            }
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            }
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            }
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            }
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            }
            Opcode::JMP => {
                let target_instruction = self.registers[self.next_8_bits() as usize];
                self.pc = target_instruction as usize;
            }
            Opcode::JMPB => {
                let offset = self.registers[self.next_8_bits() as usize];
                self.pc = self.pc - (offset as usize);
            }
            Opcode::JMPF => {
                let offset = self.registers[self.next_8_bits() as usize];
                self.pc = self.pc + (offset as usize);
            }
            Opcode::JMPC => {
                let target_instruction = self.registers[self.next_8_bits() as usize];
                if self.conditional {
                    self.pc = target_instruction as usize;
                }
            }
            Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.conditional = register1 == register2;
            }
            Opcode::NEQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.conditional = register1 != register2;
            }
            Opcode::GT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.conditional = register1 > register2;
            }
            Opcode::LT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.conditional = register1 < register2;
            }
            Opcode::GTQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.conditional = register1 >= register2;
            }
            Opcode::LTQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.conditional = register1 <= register2;
            }
            Opcode::NOP => {
                // No code on a no-op
                // ;)))
            }
            _ => {
                println!("Unrecognized opcode found! Terminating");
                return false;
            }
        }
        return true;
    }

    pub fn run(&mut self) {
        let mut running = true;
        while running {
            running = self.execute_instruction();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::program_parsers::assemble;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0);
        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_nop() {
        let mut test_vm = VM::new();
        test_vm.program = vec![255, 255, 255];
        test_vm.run();
        assert_eq!(test_vm.pc, 3);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::new();
        test_vm.program = vec![
            1, 0, 1, 244, // LOAD $0 #500
            1, 1, 0, 10, //  LOAD $1 #10
            2, 0, 1, 0, //   ADD  $0 $1 $0
        ];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 510);
    }

    #[test]
    fn test_opcode_sub() {
        let mut test_vm = VM::new();
        test_vm.program = vec![
            1, 0, 1, 244, // LOAD $0 #500
            1, 1, 0, 10, //  LOAD $1 #10
            3, 0, 1, 0, //   SUB  $0 $1 $0
        ];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 490);
    }

    #[test]
    fn test_opcode_mul() {
        let mut test_vm = VM::new();
        test_vm.program = vec![
            1, 0, 1, 244, // LOAD $0 #500
            1, 1, 0, 10, //  LOAD $1 #10
            4, 0, 1, 3, //   SUB  $0 $1 $0
        ];
        test_vm.run();
        assert_eq!(test_vm.registers[3], 5000);
    }

    #[test]
    fn test_opcode_div() {
        let mut test_vm = VM::new();
        test_vm.program = vec![
            1, 0, 1, 244, // LOAD $0 #500
            1, 1, 0, 6, //   LOAD $1 #6
            5, 0, 1, 3, //   DIV  $0 $1 $0
        ];
        test_vm.run();
        assert_eq!(test_vm.registers[3], 83);
        assert_eq!(test_vm.remainder, 2);
    }

    #[test]
    fn test_opcode_jmp() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.step();
        assert_eq!(test_vm.pc, 5);
    }

    #[test]
    fn test_opcode_jmpb() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 6;
        test_vm.pc = 1;
        test_vm.program = vec![0, 255, 255, 255, 7, 0];
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_jmpf() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.program = vec![255, 8, 0];
        test_vm.step();
        test_vm.step();
        assert_eq!(test_vm.pc, 8);
    }

    #[test]
    fn test_opcode_jmpc() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.program = vec![9, 0];
        test_vm.step();
        assert_eq!(test_vm.pc, 2);
        test_vm.pc = 0;
        test_vm.conditional = true;
        test_vm.step();
        assert_eq!(test_vm.pc, 5);
    }

    #[test]
    fn test_opcode_eq() {
        let mut test_vm = VM::new();
        test_vm.program = vec![10, 0, 1];
        test_vm.registers[0] = 3;
        test_vm.registers[1] = 3;
        test_vm.step();
        assert_eq!(test_vm.conditional, true);
        test_vm.pc = 0;
        test_vm.registers[1] = 4;
        test_vm.step();
        assert_eq!(test_vm.conditional, false);
    }

    #[test]
    fn test_opcode_neq() {
        let mut test_vm = VM::new();
        test_vm.program = vec![11, 0, 1];
        test_vm.registers[0] = 3;
        test_vm.registers[1] = 3;
        test_vm.step();
        assert_eq!(test_vm.conditional, false);
        test_vm.pc = 0;
        test_vm.registers[1] = 4;
        test_vm.step();
        assert_eq!(test_vm.conditional, true);
    }

    #[test]
    fn test_opcode_gt() {
        let mut test_vm = VM::new();
        test_vm.program = vec![12, 0, 1];
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 3;
        test_vm.step();
        assert_eq!(test_vm.conditional, true);
        test_vm.pc = 0;
        test_vm.registers[1] = 7;
        test_vm.step();
        assert_eq!(test_vm.conditional, false);
    }

    #[test]
    fn test_opcode_lt() {
        let mut test_vm = VM::new();
        test_vm.program = vec![13, 0, 1];
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 3;
        test_vm.step();
        assert_eq!(test_vm.conditional, false);
        test_vm.pc = 0;
        test_vm.registers[1] = 7;
        test_vm.step();
        assert_eq!(test_vm.conditional, true);
    }

    #[test]
    fn test_opcode_gtq() {
        let mut test_vm = VM::new();
        test_vm.program = vec![14, 0, 1];
        test_vm.registers[0] = 3;
        test_vm.registers[1] = 3;
        test_vm.step();
        assert_eq!(test_vm.conditional, true);
        test_vm.pc = 0;
        test_vm.registers[1] = 2;
        test_vm.step();
        assert_eq!(test_vm.conditional, true);
        test_vm.pc = 0;
        test_vm.registers[1] = 5;
        test_vm.step();
        assert_eq!(test_vm.conditional, false);
    }

    #[test]
    fn test_opcode_ltq() {
        let mut test_vm = VM::new();
        test_vm.program = vec![15, 0, 1];
        test_vm.registers[0] = 3;
        test_vm.registers[1] = 3;
        test_vm.step();
        assert_eq!(test_vm.conditional, true);
        test_vm.pc = 0;
        test_vm.registers[1] = 5;
        test_vm.step();
        assert_eq!(test_vm.conditional, true);
        test_vm.pc = 0;
        test_vm.registers[0] = 2;
        test_vm.step();
        assert_eq!(test_vm.conditional, true);
    }

    #[test]
    fn test_fib() {
        let mut test_vm = VM::new();
        test_vm.program = vec![
            1, 0, 0, 0, //   load $0 #0
            1, 1, 0, 1, //   load $1 #1
            1, 2, 0, 0, //   load $2 #0
            1, 3, 0, 0, //   load $3 #0
            1, 4, 0, 0, //   load $4 #0   <- fjöldi ítrana
            1, 5, 0, 1, //   load $5 #1   <- tala til að incrementa með
            1, 6, 0, 20, //  load $6 #10  <- hámarksfjöldi ítrana
            2, 1, 3, 2, //   add $1 $3 $2 <- mov 1 => 2
            2, 0, 1, 1, //   add $0 $1 $1
            2, 2, 3, 0, //   add $2 $3 $0 <- mov 2 => 0
            2, 4, 5, 4, //   add $4 $5 $4 <- inc 4
            13, 4, 6, //     lt $4 $6
            1, 7, 0, 28, //  load $7 $28
            9, 7, //         jmpc $7
            0, //            hlt
        ];
        test_vm.run();
        assert_eq!(test_vm.registers[1], 10946);
    }

    #[test]
    fn test_assembly_program() {
        let mut test_vm = VM::new();
        test_vm.program = assemble("load $0 #100\nload $1 #50\nmul $0 $1 $0\nhlt".to_string());
        test_vm.run();
        assert_eq!(test_vm.registers[0], 5000);
    }
}
