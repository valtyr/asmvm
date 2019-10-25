use crate::assembler::Token;
use nom::types::CompleteStr;
use nom::*;

named!(
    pub register <CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("$") >>
            number: digit >>
            (
                Token::Register {
                    number: number.parse::<u8>().unwrap()
                }
            )
        )
    )
);

mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = register(CompleteStr("$0"));
        assert_eq!(result.is_ok(), true);
        let (_rest, token) = result.unwrap();
        assert_eq!(token, Token::Register { number: 0 });
        let result = register(CompleteStr("0$"));
        assert_eq!(result.is_ok(), false);
        let result = register(CompleteStr("$a"));
        assert_eq!(result.is_ok(), false);
        let result = register(CompleteStr("$23"));
        let (_rest, token) = result.unwrap();
        assert_eq!(token, Token::Register { number: 23 });
    }
}
