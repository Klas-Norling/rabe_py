//! LSW implementation of structs and types


use rabe::schemes::lsw::{
    KpAbePublicKey,
    KpAbeMasterKey,
    KpAbeSecretKey,
    KpAbeCiphertext,
};

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

use crate::serializable;

/// Wrapper for LSW public key
#[pyclass]
#[derive(serde::Serialize,serde::Deserialize)]
pub struct PyKpAbePublicKey {
    pub pk: KpAbePublicKey,
}

/// Wrapper for LSW master key
#[pyclass]
#[derive(serde::Serialize,serde::Deserialize)]
pub struct PyKpAbeMasterKey {
    pub msk: KpAbeMasterKey,
}

/// Wrapper for LSW secret key
#[pyclass]
#[derive(serde::Serialize,serde::Deserialize)]
pub struct PyKpAbeSecretKey {
    pub sk: KpAbeSecretKey,
}

/// Wrapper for LSW ciphertext
#[pyclass]
#[derive(serde::Serialize,serde::Deserialize)]
pub struct PyKpAbeCiphertext {
    pub ct: KpAbeCiphertext,
}

serializable!(PyKpAbePublicKey, PyKpAbeMasterKey, PyKpAbeSecretKey, PyKpAbeCiphertext);