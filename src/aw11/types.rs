//! [AW11](rabe::schemes::aw11) type wrappers
//! ---

use rabe::schemes::aw11::{
    Aw11GlobalKey,
    Aw11PublicKey,
    Aw11MasterKey,
    Aw11Ciphertext,
    Aw11SecretKey,
};

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

use crate::serializable;

/// Wrapper for [AW11 Global Key](Aw11GlobalKey)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyAw11GlobalKey {
    pub(crate) gk: Aw11GlobalKey,
}

/// Wrapper for [AW11 Public Key](Aw11PublicKey)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyAw11PublicKey {
    pub(crate) pks: Vec<Aw11PublicKey>,
}

/// Wrapper for [AW11 Master Key](Aw11MasterKey)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyAw11MasterKey {
    pub(crate) msk: Aw11MasterKey,
}

/// Wrapper for [AW11 Cipher Text](Aw11Ciphertext)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyAw11Ciphertext {
    pub(crate) ct: Aw11Ciphertext,
}

/// Wrapper for [AW11 Secret Key](Aw11SecretKey)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyAw11SecretKey {
    pub(crate) sk: Aw11SecretKey,
}

serializable!(PyAw11GlobalKey, PyAw11PublicKey, PyAw11MasterKey, PyAw11Ciphertext, PyAw11SecretKey);