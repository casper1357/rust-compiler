mod token;
mod lex;

pub use token::Token;
pub use lex::lex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_numbers1() {
        let input = "1+2";
        let tokens = lex(input);
        assert_eq!(tokens, vec![Token::Number(1), Token::Plus, Token::Number(2)])
    }

    #[test]
    fn lex_numbers2() {
        let input = "11*2";
        let tokens = lex(input);
        assert_eq!(tokens, vec![Token::Number(11), Token::Mul, Token::Number(2)])
    }
}