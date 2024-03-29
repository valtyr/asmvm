use crate::instruction::Opcode;

pub mod instruction_parsers;
pub mod opcode_parsers;
pub mod operand_parsers;
pub mod program_parsers;
pub mod register_parsers;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { number: u8 },
    IntegerOperand { value: i32 },
}
