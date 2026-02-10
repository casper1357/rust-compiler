use rust_compiler::lexer::*;
use rust_compiler::parser::*;
use rust_compiler::codegen::*;

use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::OptimizationLevel;

fn run_llvm_ir(ir: &str) -> i64 {
    let context = Context::create();
    let module = context.create_module("jit");
    module.set_triple(&inkwell::targets::TargetMachine::get_default_triple());

    module.parse_ir_from_string(ir).unwrap();

    let engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();

    unsafe {
        let func = engine.get_function::<unsafe extern "C" fn() -> i64>("main").unwrap();
        func.call()
    }
}

fn main() {
    let input = "1 + 2 * 3";
    let tokens = lex(input);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_expr();

    let mut builder = IRBuilder::new();
    let last = codegen_expr(&mut builder, &ast);
    builder.finalize_main(&last);
    println!("{}", builder.code);
    let result = run_llvm_ir(&builder.code);
    assert_eq!(result, 7); 
}