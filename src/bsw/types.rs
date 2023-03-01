//! All types for BWS in python wrapper

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use rabe::schemes::bsw::{CpAbeCiphertext, CpAbeMasterKey, CpAbePublicKey, CpAbeSecretKey};

use crate::serializable;

/// Wrapper for [BSW Ciphertext](CpAbeCiphertext)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBswCiphertext {
    pub ct: CpAbeCiphertext,
}

#[pymethods]
impl PyBswCiphertext {
    #[new]
    /// Deserialize an instance from a string of JSON text
    fn __init__(value: String) -> PyResult<Self> {
        Ok(Self {
            ct: match serde_json::from_str(&value) {
                Ok(ct) => ct,
                Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
            },
        })
    }

    /// Serialize the given data structure as a String of json
    fn __str__(&self) -> PyResult<String> {
        let value = match serde_json::to_string(&self.ct) {
            Ok(value) => value,
            Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
        };
        Ok(value)
    }
}

/// Wrapper for [BSW Masterkey](CpAbeMasterKey)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBswCpAbeMasterKey {
    pub msk: CpAbeMasterKey,
}

#[pymethods]
impl PyBswCpAbeMasterKey {
    #[new]
    /// Deserialize an instance from a string of JSON text
    fn __init__(value: String) -> PyResult<Self> {
        Ok(Self {
            msk: match serde_json::from_str(&value) {
                Ok(msk) => msk,
                Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
            },
        })
    }

    /// Serialize the given data structure as a String of json
    fn __str__(&self) -> PyResult<String> {
        let value = match serde_json::to_string(&self.msk) {
            Ok(value) => value,
            Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
        };
        Ok(value)
    }
}

/// Wrapper for [BSW Public Key](CpAbePublicKey)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBswCpAbePublicKey {
    pub pk: CpAbePublicKey,
}

#[pymethods]
impl PyBswCpAbePublicKey {
    #[new]
    /// Deserialize an instance from a string of JSON text
    fn __init__(value: String) -> PyResult<Self> {
        Ok(Self {
            pk: match serde_json::from_str(&value) {
                Ok(pk) => pk,
                Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
            },
        })
    }

    /// Serialize the given data structure as a String of json
    fn __str__(&self) -> PyResult<String> {
        let value = match serde_json::to_string(&self.pk) {
            Ok(value) => value,
            Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
        };
        Ok(value)
    }
}

/// Wrapper for [BSW Secret key](CpAbeSecretKey)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyBswCpAbeSecretKey {
    pub sk: CpAbeSecretKey,
}

#[pymethods]
impl PyBswCpAbeSecretKey {
    #[new]
    /// Deserialize an instance from a string of JSON text
    fn __init__(value: String) -> PyResult<Self> {
        Ok(Self {
            sk: match serde_json::from_str(&value) {
                Ok(sk) => sk,
                Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
            },
        })
    }

    /// Serialize the given data structure as a String of json
    fn __str__(&self) -> PyResult<String> {
        let value = match serde_json::to_string(&self.sk) {
            Ok(value) => value,
            Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
        };
        Ok(value)
    }
}

serializable!(
    PyBswCiphertext,
    PyBswCpAbeMasterKey,
    PyBswCpAbePublicKey,
    PyBswCpAbeSecretKey
);