//! All types for BSW in python wrapper

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use rabe::schemes::bsw::{CpAbeCiphertext, CpAbeMasterKey, CpAbePublicKey, CpAbeSecretKey};

use crate::serializable;

/// Wrapper for [BSW Ciphertext](CpAbeCiphertext)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBswCiphertext {
    pub(crate) ct: CpAbeCiphertext,
}

/// Wrapper for [BSW Masterkey](CpAbeMasterKey)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBswCpAbeMasterKey {
    pub(crate) msk: CpAbeMasterKey,
}


/// Wrapper for [BSW Public Key](CpAbePublicKey)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBswCpAbePublicKey {
    pub(crate) pk: CpAbePublicKey,
}


/// Wrapper for [BSW Secret key](CpAbeSecretKey)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBswCpAbeSecretKey {
    pub(crate) sk: CpAbeSecretKey,
}


serializable!(
    PyBswCiphertext,
    PyBswCpAbeMasterKey,
    PyBswCpAbePublicKey,
    PyBswCpAbeSecretKey
);