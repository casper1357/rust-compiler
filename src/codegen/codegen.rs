use crate::ast::Expr;

pub fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Add(a, b) => return eval(a) + eval(b),
        Expr::Mul(a, b) => return eval(a) * eval(b),
    }
}