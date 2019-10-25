use nom::types::CompleteStr;
use nom::*;

use crate::assembler::instruction_parsers::{instruction, AssemblerInstruction};

pub struct Program {
    instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut program = vec![];
        for instruction in &self.instructions {
            program.append(&mut instruction.to_bytes());
        }
        program
    }
}

pub fn assemble(code: String) -> Vec<u8> {
    let result = program(CompleteStr(&code));
    if !result.is_ok() {
        // TODO: Replace with good error handling
        std::process::exit(1);
    }
    let (_rest, program) = result.unwrap();
    program.to_bytes()
}

named!(
    pub program<CompleteStr, Program>,
    do_parse!(
        instructions: many1!(instruction) >>
        (
            Program {
                instructions: instructions,
            }
        )
    )
);

mod tests {
    use super::*;

    #[test]
    fn test_parse_program() {
        let result = program(CompleteStr("load $0 #100\n\n"));
        assert_eq!(result.is_ok(), true);
        let (rest, program) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(1, program.instructions.len());
    }

    #[test]
    fn test_program_to_bytes() {
        let result = program(CompleteStr("load $0 #100\nload $1 #500\n"));
        let (_rest, program) = result.unwrap();
        assert_eq!(program.to_bytes(), [1, 0, 0, 100, 1, 1, 1, 244]);
    }

    #[test]
    fn test_other_program_to_bytes() {
        let result = program(CompleteStr(
            "load $0 #100\nload $1 #500\ndiv $0 $1 $3\nltq $0 $1\nhlt",
        ));
        let (_rest, program) = result.unwrap();
        assert_eq!(
            program.to_bytes(),
            [
                1, 0, 0, 100, //
                1, 1, 1, 244, //
                5, 0, 1, 3, //
                15, 0, 1, //
                0, //
            ]
        )
    }
}
