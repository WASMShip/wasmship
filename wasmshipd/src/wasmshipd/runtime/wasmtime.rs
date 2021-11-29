use super::types::{
    FunctionExport, FunctionExports, RuntimeError, RuntimeResult, Value, ValueType,
};
use super::AbstractRuntime;
use wasmtime::*;
use std::path::Path;

pub struct Wasmtime {
    module: Module,
    store: Store<()>,
    functions: FunctionExports,
}

// Implementation of the `AbstractRuntime` trait
impl AbstractRuntime for Wasmtime {
    fn new(path: &Path) -> RuntimeResult<Wasmtime> {
        let engine = Engine::default();
        let module = Module::from_file(&engine, path).unwrap();
        let store = Store::new(&engine, ());
        Ok(Wasmtime {
            module,
            store,
            functions: FunctionExports::new(),
        })
    }

    fn function_exports(&self) -> RuntimeResult<FunctionExports> {
        let mut func_exports = FunctionExports::new();
        let exports = self.module.exports();
        for export in exports {
            if let Some(func) = export.ty().func() {
                let mut params = Vec::new();
                let mut results = Vec::new();
                for param in func.params() {
                    params.push(ValueType::from(param));
                }
                for result in func.results() {
                    results.push(ValueType::from(result));
                }
                func_exports.insert(
                    export.name().to_string(),
                    FunctionExport { params, results },
                );
            }
        }
        Ok(func_exports)
    }

    fn invoke(&mut self, function: &str, parameters: Vec<String>) -> RuntimeResult<Vec<Value>> {
        if self.functions.is_empty() {
            self.functions = self.function_exports()?;
        }
        let functions = self.functions.clone();
        let function_export = match functions.get(function) {
            Some(function_export) => function_export,
            None => {
                return Err(RuntimeError::ExecutionError(format!(
                    "function {} not found",
                    function
                )))
            }
        };
        let function_params = function_export.params.clone();
        if parameters.len() != function_params.len() {
            return Err(RuntimeError::ExecutionError(format!(
                "function {} params count not match {}/{}",
                function,
                parameters.len(),
                function_params.len()
            )));
        }
        let function_results = function_export.results.clone();
        let instance = Instance::new(&mut self.store, &self.module, &[])?;
        let func = instance.get_func(&mut self.store, function).unwrap();
        let mut params = vec![Val::null(); function_params.len()];
        let mut results = vec![Val::null(); function_results.len()];
        for (i, param) in function_params.iter().enumerate() {
            match param {
                ValueType::I32 => {
                    params[i] = Val::I32(parameters[i].parse()?);
                }
            }
        }
        match func.call(&mut self.store, &params, &mut results) {
            Ok(_) => {
                let mut values = Vec::new();
                for result in results {
                    match result {
                        Val::I32(x) => {
                            values.push(Value::I32(x));
                        }
                        _ => {
                            return Err(RuntimeError::ExecutionError(format!(
                                "function {} result type not match",
                                function
                            )));
                        }
                    }
                }
                Ok(values)
            }
            Err(err) => Err(RuntimeError::ExecutionError(err.to_string())),
        }
    }
}
