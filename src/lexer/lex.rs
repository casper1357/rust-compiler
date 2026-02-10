use crate::lexer::Token;

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => {
                let mut num = 0;
                while let Some(&d) = chars.peek() {
                    if d.is_digit(10) {
                        num = num * 10 + d.to_digit(10).unwrap() as i64;
                        chars.next();
                    }
                    else {
                        break;
                    }
                }
                tokens.push(Token::Number(num))
            }

            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }

            '*' => {
                chars.next();
                tokens.push(Token::Mul);
            }

            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            }

            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            }

            _ => {panic!("Unexpected character in the parser: {}", c);}
        }
    }

    return tokens
}