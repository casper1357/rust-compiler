#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LLVMType {
    I32,
    I64,
    F32,
    F64,
}

impl LLVMType {
    pub fn as_str(&self) -> &str {
        match self {
            LLVMType::I32 => "i32",
            LLVMType::I64 => "i64",
            LLVMType::F32 => "float",
            LLVMType::F64 => "double",
        }
    }
}