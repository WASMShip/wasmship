use std::collections::HashMap;
use std::num::ParseIntError;
pub type FunctionExports = HashMap<String, FunctionExport>;
pub type RuntimeResult<T> = Result<T, RuntimeError>;

/// Enum for parameters and return values
/// 
/// As in different runtimes, they may provide their own type for parameters and return values. We provide a generic type and conversion functions for them.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    I32(i32),
    // TODO: Add more types
}

/// Enum for parameter and return value types
#[derive(Debug, Clone)]
pub enum ValueType {
    I32,
    // TODO: Add more types
}

impl From<wasmtime::ValType> for ValueType {
    fn from(vtype: wasmtime::ValType) -> Self {
        match vtype {
            wasmtime::ValType::I32 => ValueType::I32,
            // TODO: Add more types
            _ => unimplemented!(),
        }
    }
}

/// Describes an exported function's parameters and return values
#[derive(Debug, Clone)]
pub struct FunctionExport {
    pub params: Vec<ValueType>,
    pub results: Vec<ValueType>,
}

/// Error from runtime
#[derive(Debug)]
pub enum RuntimeError {
    /// This should be used when the runtime failed to execute the function
    ExecutionError(String),
    // TODO: Add more error type
}

impl From<ParseIntError> for RuntimeError {
    fn from(err: ParseIntError) -> RuntimeError {
        RuntimeError::ExecutionError(err.to_string())
    }
}

impl From<anyhow::Error> for RuntimeError {
    fn from(err: anyhow::Error) -> RuntimeError {
        RuntimeError::ExecutionError(err.to_string())
    }
}
