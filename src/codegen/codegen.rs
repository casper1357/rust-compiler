use crate::{ast::Expr, codegen::irbuilder::IRBuilder, codegen::irbuilder::Value, codegen::LLVMType};


pub fn codegen_expr(builder: &mut IRBuilder, expr: &Expr) -> Value {
    match expr {
        Expr::Number(n) => builder.const_int(*n, LLVMType::I64),
        Expr::Add(lhs, rhs) => {
            let l = codegen_expr(builder, lhs);
            let r = codegen_expr(builder, rhs);
            builder.add(&l, &r)
        }
        Expr::Mul(lhs, rhs) => {
            let l = codegen_expr(builder, lhs);
            let r = codegen_expr(builder, rhs);
            builder.mul(&l, &r)
        }
    }
}