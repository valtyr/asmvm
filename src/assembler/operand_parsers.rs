use crate::assembler::Token;
use nom::types::CompleteStr;
use nom::*;

named!(
    pub integer_operand<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("#") >>
            number: digit >>
            (
                Token::IntegerOperand{value: number.parse::<i32>().unwrap()}
            )
        )
    )
);

pub mod tests {
    use super::*;

    #[test]
    fn test_parse_integer_operand() {
        let result = integer_operand(CompleteStr("#146"));
        assert_eq!(result.is_ok(), true);
        let (_rest, token) = result.unwrap();
        assert_eq!(token, Token::IntegerOperand { value: 146 });
        let result = integer_operand(CompleteStr("55#"));
        assert_eq!(result.is_ok(), false);
        let result = integer_operand(CompleteStr("#"));
        assert_eq!(result.is_ok(), false);
    }
}
