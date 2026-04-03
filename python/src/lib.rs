#![deny(missing_docs)]
#![deny(warnings)]

//! # **rsonschema**
//!
//! A fast, simple, user-friendly JSON Schema validator for Python

use pyo3::prelude::*;
use pythonize::depythonize;
use serde_json::Value;

/// The validation error
#[pyclass]
#[derive(Debug, Eq, PartialEq)]
pub struct ValidationError {
    /// The pointer to the value that caused the error
    pub pointer: Vec<String>,

    /// The error message
    pub message: String,

    /// The value that caused the error
    pub instance: Value,
}

#[pymethods]
impl ValidationError {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        self.message.to_string()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.pointer == other.pointer
            && self.message == other.message
            && self.instance == other.instance
    }

    #[new]
    #[pyo3(signature = (pointer, message, instance))]
    fn new(pointer: Vec<String>, message: String, instance: &Bound<'_, PyAny>) -> Self {
        let instance = depythonize(instance).unwrap();
        Self {
            pointer,
            message,
            instance,
        }
    }
}

impl From<rsonschema::error::ValidationError> for ValidationError {
    fn from(err: rsonschema::error::ValidationError) -> Self {
        Self {
            pointer: err
                .pointer
                .iter()
                .map(|pointer| pointer.to_string())
                .collect(),
            message: err.to_string(),
            instance: err.instance,
        }
    }
}

#[pyfunction]
#[pyo3(signature = (instance, schema, pointer=None, ref_resolver=None))]
fn validate(
    instance: &Bound<'_, PyAny>,
    schema: &Bound<'_, PyAny>,
    pointer: Option<&str>,
    ref_resolver: Option<&Bound<'_, PyAny>>,
) -> Vec<ValidationError> {
    let instance: Value = depythonize(instance).unwrap();
    let schema: Value = depythonize(schema).unwrap();
    let report = match ref_resolver {
        Some(ref_resolver) => {
            let ref_resolver = |ref_: &str| -> Option<Value> {
                let schema = ref_resolver.call1((ref_,)).unwrap();
                depythonize(&schema).unwrap()
            };
            rsonschema::validate_with_resolver(&instance, schema, pointer, Some(&ref_resolver))
        }
        None => rsonschema::validate_with_resolver(&instance, schema, pointer, None),
    };
    report
        .errors
        .unwrap_or_default()
        .into_iter()
        .map(|err| err.into())
        .collect()
}

#[pymodule]
#[pyo3(name = "rsonschema")]
fn _rsoschema(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(validate, module)?)?;
    module.add_class::<ValidationError>()?;
    Ok(())
}
