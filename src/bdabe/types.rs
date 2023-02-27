//! [Bdabe](rabe::schemes::bdabe) type wrappers
//! ---

use rabe::schemes::bdabe::{
    BdabeCiphertext, BdabeMasterKey, BdabePublicAttributeKey, BdabePublicKey, BdabePublicUserKey,
    BdabeSecretAttributeKey, BdabeSecretAuthorityKey, BdabeUserKey,
};

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::serializable;

/// Wrapper for [BDABE Public Key](BdabePublicKey)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBdabePublicKey {
    pub(crate) pk: BdabePublicKey,
}

/// Wrapper for [BDABE Master Key](BdabeMasterKey)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBdabeMasterKey {
    pub(crate) mk: BdabeMasterKey,
}

/// Wrapper for [BDABE Cipher Text](BdabeCiphertext)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBdabeCiphertext {
    pub(crate) ct: BdabeCiphertext,
}

/// Wrapper for [BDABE Public attribute Key](BdabePublicAttributeKey)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBdabePublicAttributeKey {
    pub(crate) pak: Vec<BdabePublicAttributeKey>,
}

/// Wrapper for [BDABE Secret authority key](BdabeSecretAuthorityKey)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBdabeSecretAuthorityKey {
    pub(crate) sak: BdabeSecretAuthorityKey,
}

/// Wrapper for [BDABE Public user Key](BdabePublicUserKey)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBdabePublicUserKey {
    pub(crate) puk: BdabePublicUserKey,
}

/// Wrapper for [BDABE User key](BdabeUserKey)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBdabeUserKey {
    pub(crate) uk: BdabeUserKey,
}
/// Wrapper for [BDABE Secret attribute key](BdabeSecretAttributeKey)
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