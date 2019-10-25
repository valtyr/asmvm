#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Opcode {
    HLT,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JMPB,
    JMPF,
    JMPC,
    EQ,
    NEQ,
    GT,
    LT,
    GTQ,
    LTQ,
    IGL,
    NOP,
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => return Opcode::HLT,
            1 => return Opcode::LOAD,
            2 => return Opcode::ADD,
            3 => return Opcode::SUB,
            4 => return Opcode::MUL,
            5 => return Opcode::DIV,
            6 => return Opcode::JMP,
            7 => return Opcode::JMPB,
            8 => return Opcode::JMPF,
            9 => return Opcode::JMPC,
            10 => return Opcode::EQ,
            11 => return Opcode::NEQ,
            12 => return Opcode::GT,
            13 => return Opcode::LT,
            14 => return Opcode::GTQ,
            15 => return Opcode::LTQ,
            255 => return Opcode::NOP,
            _ => return Opcode::IGL,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode: opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let test_opcode = Opcode::HLT;
        assert_eq!(test_opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let test_instruction = Instruction::new(Opcode::HLT);
        assert_eq!(test_instruction.opcode, Opcode::HLT);
    }
}
