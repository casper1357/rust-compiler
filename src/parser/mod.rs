mod parser;

pub use parser::Parser;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Token;
    use crate::ast::Expr;

    #[test]
    fn parse_numbers1() {
        let tokens = vec![Token::Number(1), Token::Plus, Token::Number(2)];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expr();
        assert_eq!(ast, Expr::Add(Box::new(Expr::Number(1)), Box::new(Expr::Number(2))))
    }

}