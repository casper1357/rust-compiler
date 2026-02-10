use crate::codegen::LLVMType;
use std::fmt::Write;

pub struct IRBuilder {
    pub code: String,
    temp_counter: usize,
}

pub struct Value {
    name: String,
    ty: LLVMType,
}

impl IRBuilder {
    pub fn new() -> Self {
        Self {code: String::new(), temp_counter: 0}
    }

    fn next_temp(&mut self) -> String {
        let t = format!("%{}", self.temp_counter);
        self.temp_counter += 1;

        t
    }

    pub fn add(&mut self, lhs: &Value, rhs: &Value) -> Value {
        let temp = self.next_temp();
        writeln!(
            self.code,
            "{} = add {} {}, {}",
            temp,
            lhs.ty.as_str(),
            lhs.name,
            rhs.name
        )
        .unwrap();
        Value { name: temp, ty: lhs.ty}
    }

    pub fn mul(&mut self, lhs: &Value, rhs: &Value) -> Value {
        let temp = self.next_temp();
        writeln!(
            self.code,
            "{} = mul {} {}, {}",
            temp,
            lhs.ty.as_str(),
            lhs.name,
            rhs.name
        )
        .unwrap();
        Value { name: temp, ty: lhs.ty}
    }

    pub fn const_int(&mut self, n: i64, ty: LLVMType) -> Value {
        let temp = self.next_temp();
        writeln!(self.code, "{} = add {} 0, {}", temp, ty.as_str(), n).unwrap();
        Value { name: temp, ty }
    }

    pub fn const_float(&mut self, n: f64, ty: LLVMType) -> Value {
        let temp = self.next_temp();
        writeln!(self.code, "{} = fadd {} 0.0, {}", temp, ty.as_str(), n).unwrap();
        Value { name: temp, ty }
    }

    pub fn finalize_main(&mut self, last: &Value) {
        self.code.push_str(&format!("ret i64 {}\n", last.name));
    }
    

}