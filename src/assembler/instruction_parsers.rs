use crate::assembler::opcode_parsers::*;
use crate::assembler::operand_parsers::integer_operand;
use crate::assembler::register_parsers::register;
use crate::assembler::Token;

use nom::types::CompleteStr;
use nom::*;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        match &self.opcode {
            Token::Op { code } => match code {
                _ => results.push(*code as u8),
            },
            _ => {
                // TODO: Proper error handling
                println!("Non-opcode found in opcode field");
                std::process::exit(1);
            }
        }

        for operand in vec![&self.operand1, &self.operand2, &self.operand3] {
            match operand {
                Some(t) => AssemblerInstruction::extract_operand(t, &mut results),
                None => {}
            }
        }

        return results;
    }

    pub fn extract_operand(t: &Token, results: &mut Vec<u8>) {
        match t {
            Token::Register { number } => {
                results.push(*number);
            }
            Token::IntegerOperand { value } => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            _ => {
                // TODO: Proper error handling
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        }
    }
}

named!(
    pub instruction_o_r_i<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        r: register >>
        i: integer_operand >>
        opt!(multispace) >>
        (
            AssemblerInstruction{
                opcode: o,
                operand1: Some(r),
                operand2: Some(i),
                operand3: None,
            }
        )
    )
);

named!(
    pub instruction_o_r_r<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        r1: register >>
        r2: register >>
        opt!(multispace) >>
        (
            AssemblerInstruction{
                opcode: o,
                operand1: Some(r1),
                operand2: Some(r2),
                operand3: None,
            }
        )
    )
);

named!(instruction_o<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        opt!(multispace) >>
        (
            AssemblerInstruction{
                opcode: o,
                operand1: None,
                operand2: None,
                operand3: None,
            }
        )
    )
);

named!(instruction_o_r_r_r<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        r1: register >>
        r2: register >>
        r3: register >>
        opt!(multispace) >>
        (
            AssemblerInstruction{
                opcode: o,
                operand1: Some(r1),
                operand2: Some(r2),
                operand3: Some(r3),
            }
        )
    )
);

named!(
    pub instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt_complete!(
            instruction_o_r_r_r |
            instruction_o_r_i |
            instruction_o_r_r |
            instruction_o 
        ) >> (ins)
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction_load() {
        let result = instruction(CompleteStr("load $0 #100\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::LOAD },
                    operand1: Some(Token::Register { number: 0 }),
                    operand2: Some(Token::IntegerOperand { value: 100 }),
                    operand3: None
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_halt() {
        let result = instruction(CompleteStr("hlt\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::HLT },
                    operand1: None,
                    operand2: None,
                    operand3: None
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_mul() {
        let result = instruction(CompleteStr("mul $1 $2 $3\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::MUL },
                    operand1: Some(Token::Register { number: 1 }),
                    operand2: Some(Token::Register { number: 2 }),
                    operand3: Some(Token::Register { number: 3 }),
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_lt() {
        let result = instruction(CompleteStr("lt $1 $2\n eq $1 $2\n load $1 #123"));
        let (_rest, instruction) = result.unwrap();
        assert_eq!(
            instruction,
            AssemblerInstruction {
                opcode: Token::Op { code: Opcode::LT },
                operand1: Some(Token::Register { number: 1 }),
                operand2: Some(Token::Register { number: 2 }),
                operand3: None,
            }
        )
    }
}
