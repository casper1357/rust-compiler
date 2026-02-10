use crate::ast::Expr;
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {tokens, pos: 0}
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next (&mut self) -> Option<Token> {
        let token = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        return token
    }

    pub fn parse_expr(&mut self) -> Expr {
        let mut node = self.parse_term();
        
        while let Some(Token::Plus) = self.peek() {
            self.next();
            let rhs = self.parse_term();
            node = Expr::Add(Box::new(node), Box::new(rhs));
        }

        return node
    }

    fn parse_term(&mut self) -> Expr {
        let mut node = self.parse_factor();
        
        while let Some(Token::Mul) = self.peek() {
            self.next();
            let rhs = self.parse_factor();
            node = Expr::Mul(Box::new(node), Box::new(rhs));
        }

        return node
    }

    fn parse_factor(&mut self) -> Expr {
        match self.next() {
            Some(Token::Number(n)) => return Expr::Number(n),
            Some(Token::LParen) => {
                let expr = self.parse_expr();
                match self.next() {
                    Some(Token::RParen) => return expr, 
                    _ => panic!("Unexpected ')'"),
                }

            }
            _ => panic!("Unexpeced number or '('")
        }
    }
}  