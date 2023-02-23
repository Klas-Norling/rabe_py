//! [Bdabe](rabe::schemes::bdabe) type wrappers
//! ---

use rabe::schemes::bdabe::{
    BdabePublicKey,
    BdabeMasterKey,
    BdabeUserKey,
    BdabePublicUserKey,
    BdabePublicAttributeKey,
    BdabeSecretAuthorityKey,
    BdabeCiphertext,
};

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

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


serializable!(PyBdabePublicKey, PyBdabeMasterKey, PyBdabeCiphertext, PyBdabePublicAttributeKey, PyBdabeSecretAuthorityKey, PyBdabePublicUserKey, PyBdabeUserKey);