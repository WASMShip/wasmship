mod types;
mod wasmtime;

use types::{FunctionExports, Module, RuntimeResult, Value};

/// Abstraction over different wasm runtimes
///
/// This trait defines what a runtime should at least provide functions. The way to implement these functions may differ.
pub trait AbstractRuntime {
    fn new(module: Module) -> RuntimeResult<Self>
    where
        Self: Sized;
    fn function_exports(&self) -> RuntimeResult<FunctionExports>;
    fn invoke(
        &mut self,
        function: Option<&str>,
        parameters: Vec<String>,
    ) -> RuntimeResult<Vec<Value>>;
}

/// Enum for currently supported runtimes
#[derive(Debug, Clone)]
pub enum SupportedRuntime {
    Wasmtime,
}

/// A gerneric runtime based on abstraction over different wasm runtimes
#[derive(Debug, Clone)]
pub struct Runtime<T>
where
    T: AbstractRuntime,
{
    rtype: SupportedRuntime,
    runtime: T,
}

impl Runtime<wasmtime::Wasmtime> {
    /// Create a new runtime
    pub fn new(rtype: SupportedRuntime, module: Module) -> Runtime<wasmtime::Wasmtime> {
        let runtime = wasmtime::Wasmtime::new(module).unwrap();
        Runtime { rtype, runtime }
    }

    /// Invoke a function in the wasm module
    ///
    /// Parameters are strings and would be converted to the correct type automatically.
    pub fn invoke(
        &mut self,
        function: Option<&str>,
        parameters: Vec<String>,
    ) -> RuntimeResult<Vec<Value>> {
        self.runtime.invoke(function, parameters)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::Modules;

    #[test]
    /// Test add two numbers module in Wasmtime
    ///
    /// Code for add module:
    /// ```
    /// #[no_mangle]
    /// pub extern "C" fn add(a: i32, b: i32) -> i32 {
    ///     a + b
    /// }
    /// ```
    fn test_wasmtime_add() {
        let modules = Modules::load("tests/modules").unwrap();
        let module = modules.get_module("mymod", "latest").unwrap();
        let mut runtime = Runtime::new(SupportedRuntime::Wasmtime, module);
        let result = runtime
            .invoke(Some("add"), vec!["300".to_string(), "206".to_string()])
            .unwrap();
        assert_eq!(result[0], Value::I32(506));
        // test when no entry function is specified
        let result = runtime
            .invoke(None, vec!["1".to_string(), "2".to_string()])
            .unwrap();
        assert_eq!(result[0], Value::I32(3));
    }
}
