//! [Bdabe](rabe::schemes::bdabe) type wrappers
//! ---

use rabe::schemes::bdabe::{
    BdabePublicKey,
    BdabeMasterKey,
    BdabeUserKey,
    BdabePublicUserKey,
    BdabeSecretUserKey,
    BdabeSecretAttributeKey,
    BdabePublicAttributeKey,
    BdabeSecretAuthorityKey,
    BdabeCiphertextTuple,
    BdabeCiphertext,
};

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

use crate::serializable;

#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBdabePublicKey {
    pub(crate) pk: BdabePublicKey,
}

#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBdabeMasterKey {
    pub(crate) mk: BdabeMasterKey,
}

#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBdabeCiphertext {
    pub(crate) ct: BdabeCiphertext,
}

#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBdabePublicAttributeKey {
    pub(crate) pak: BdabePublicAttributeKey,
}

#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBdabeSecretAuthorityKey {
    pub(crate) sak: BdabeSecretAuthorityKey,
}

#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBdabePublicUserKey {
    pub(crate) puk: BdabePublicUserKey,
}


serializable!(PyBdabePublicKey, PyBdabeMasterKey, PyBdabeCiphertext, PyBdabePublicAttributeKey, PyBdabeSecretAuthorityKey, PyBdabePublicUserKey);