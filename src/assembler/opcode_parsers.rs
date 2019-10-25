use crate::assembler::Token;
use crate::instruction::Opcode;
use nom::types::CompleteStr;
use nom::*;

impl<'a> From<CompleteStr<'a>> for Opcode {
    fn from(v: CompleteStr<'a>) -> Self {
        match v {
            CompleteStr("load") => Opcode::LOAD,
            CompleteStr("add") => Opcode::ADD,
            CompleteStr("sub") => Opcode::SUB,
            CompleteStr("mul") => Opcode::MUL,
            CompleteStr("div") => Opcode::DIV,
            CompleteStr("hlt") => Opcode::HLT,
            CompleteStr("jmp") => Opcode::JMP,
            CompleteStr("jmpf") => Opcode::JMPF,
            CompleteStr("jmpb") => Opcode::JMPB,
            CompleteStr("eq") => Opcode::EQ,
            CompleteStr("neq") => Opcode::NEQ,
            CompleteStr("gtq") => Opcode::GTQ,
            CompleteStr("gt") => Opcode::GT,
            CompleteStr("ltq") => Opcode::LTQ,
            CompleteStr("lt") => Opcode::LT,
            CompleteStr("jmpc") => Opcode::JMPC,
            CompleteStr("nop") => Opcode::NOP,
            _ => Opcode::IGL,
        }
    }
}

named!(pub opcode<CompleteStr, Token>,
  do_parse!(
      opcode: alpha1 >>
      (
        {
            Token::Op{code: Opcode::from(opcode)}
        }
      )
  )
);

mod tests {
    use super::*;

    #[test]
    fn test_opcode_load() {
        let result = opcode(CompleteStr("load"));
        assert_eq!(result.is_ok(), true);

        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, CompleteStr(""));

        let result = opcode(CompleteStr("aold"));
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::IGL });
    }
}
