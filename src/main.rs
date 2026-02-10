mod ast;
mod lexer;
mod parser;
mod codegen;

use lexer::lex;
use parser::Parser;
use codegen::eval;

fn main() {
    let input = "1+2*(3+4)";
    let tokens = lex(input);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_expr();
    let result = eval(&ast);

    println!("{:?}", ast);
    println!("Result = {}", result);

}