//! [Bdabe](rabe::schemes::bdabe) type wrappers
//! ---

use rabe::schemes::bdabe::{
    BdabeCiphertext, BdabeMasterKey, BdabePublicAttributeKey, BdabePublicKey, BdabePublicUserKey,
    BdabeSecretAttributeKey, BdabeSecretAuthorityKey, BdabeUserKey,
};

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::serializable;

/// Some doc comment
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBdabePublicKey {
    pub(crate) pk: BdabePublicKey,
}

/// Some doc comment
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBdabeMasterKey {
    pub(crate) mk: BdabeMasterKey,
}

/// Some doc comment
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBdabeCiphertext {
    pub(crate) ct: BdabeCiphertext,
}

/// Some doc comment
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBdabePublicAttributeKey {
    pub(crate) pak: Vec<BdabePublicAttributeKey>,
}

/// Some doc comment
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBdabeSecretAuthorityKey {
    pub(crate) sak: BdabeSecretAuthorityKey,
}

/// Some doc comment
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBdabePublicUserKey {
    pub(crate) puk: BdabePublicUserKey,
}

/// Some doc comments
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBdabeUserKey {
    pub(crate) uk: BdabeUserKey,
}
/// Some doc comments
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct PyBdabeSecretAttributeKey {
    pub(crate) sak: BdabeSecretAttributeKey,
}

// Implement
#[pymethods]
impl PyBdabeUserKey {
    #[new]
    /// asd
    pub fn __init__(value: String) -> PyResult<Self> {
        match serde_json::from_str(&value) {
            Ok(value) => Ok(value),
            Err(e) => Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
        }
    }
    /// asd
    pub fn __str__(&self) -> PyResult<String> {
        match serde_json::to_string(&self) {
            Ok(value) => Ok(value),
            Err(e) => Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
        }
    }

    /// ```python
    /// _u_key.append(request_attribute_sk(_u_key, _a1_key, _att1))
    /// ```
    pub fn append(&mut self, value: PyBdabeSecretAttributeKey) {
        self.uk._ska.push(value.sak);
    }
}

serializable!(
    PyBdabePublicKey,
    PyBdabeMasterKey,
    PyBdabeCiphertext,
    PyBdabePublicAttributeKey,
    PyBdabeSecretAuthorityKey,
    PyBdabePublicUserKey,
    PyBdabeSecretAttributeKey
);
