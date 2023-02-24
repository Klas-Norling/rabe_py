//! All types for AC17 CP and KP in python wrapper

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use rabe::schemes::ac17::{
    Ac17CpCiphertext, Ac17CpSecretKey, Ac17KpCiphertext, Ac17KpSecretKey, Ac17MasterKey,
    Ac17PublicKey,
};
use rabe::utils::policy::pest::PolicyLanguage;

use crate::serializable;

/// Wrapper for [Ac17 Kp ciphertext](Ac17KpCiphertext)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyAc17KpCiphertext {
    pub(crate) ct: Ac17KpCiphertext,
}

/// Wrapper for [Ac17 Cp ciphertext](Ac17CpCiphertext)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyAc17CpCiphertext {
    pub(crate) ct: Ac17CpCiphertext,
}

/// Wrapper for [Ac17 Public Key](Ac17PublicKey)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyAc17PublicKey {
    pub(crate) pk: Ac17PublicKey,
}

/// Wrapper for [Ac17 Master Key](Ac17MasterKey)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyAc17MasterKey {
    pub(crate) msk: Ac17MasterKey,
}

/// Wrapper for [Ac17 Kp Secret Key](Ac17KpSecretKey)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyAc17KpSecretKey {
    pub(crate) sk: Ac17KpSecretKey,
}

/// Wrapper for [Policy Language](PolicyLanguage)
#[pyclass]
pub struct PyPolicyLanguage {
    pub(crate) _lang: PolicyLanguage,
}

/// Wrapper for [Ac17 Cp Secret Key](Ac17CpSecretKey)
#[pyclass]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PyAc17CpSecretKey {
    pub(crate) sk: Ac17CpSecretKey,
}

serializable!(
    PyAc17KpCiphertext,
    PyAc17CpCiphertext,
    PyAc17PublicKey,
    PyAc17MasterKey,
    PyAc17KpSecretKey,
    PyAc17CpSecretKey
);
