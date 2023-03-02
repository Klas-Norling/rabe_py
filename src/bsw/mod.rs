//! The python wrapper for the BSW scheme
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use rabe::schemes::bsw::{
    decrypt as bsw_decrypt, encrypt as bsw_encrypt, keygen as bsw_keygen, setup as bsw_setup,
};

use rabe::utils::policy::pest::PolicyLanguage;

pub mod types;
use types::*;

/// Setup for BSW -> (pk, msk)
#[pyfunction]
fn setup() -> PyResult<(PyBswCpAbePublicKey, PyBswCpAbeMasterKey)> {
    let (pk, msk) = bsw_setup();
    let pk = PyBswCpAbePublicKey { pk };
    let msk = PyBswCpAbeMasterKey { msk };
    Ok((pk, msk))
}

/// Encrypt for BSW -> PyBswCiphertext
///
/// Input:
/// - pk: PyBswCpAbePublicKey
/// - policy: string
/// - message: string
#[pyfunction]
fn encrypt(pk: &PyBswCpAbePublicKey, policy: String, message: String) -> PyResult<PyBswCiphertext> {
    let message = message.into_bytes();
    let ct = match bsw_encrypt(&pk.pk, &policy, &message, PolicyLanguage::HumanPolicy) {
        Ok(ct) => ct,
        Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
    };
    let ct = PyBswCiphertext { ct };
    Ok(ct)
}

/// Keygen for BSW -> PyBswCpAbeSecretKey
///
/// Input:
/// - pk: PyBswCpAbePublicKey
/// - msk: PyBswCpAbeMasterKey
/// - attributes: list[str]
#[pyfunction]
fn keygen(
    pk: &PyBswCpAbePublicKey,
    msk: &PyBswCpAbeMasterKey,
    attributes: Vec<String>,
) -> PyResult<PyBswCpAbeSecretKey> {
    let sk = match bsw_keygen(&pk.pk, &msk.msk, &attributes) {
        Some(sk) => sk,
        None => return Err(PyErr::new::<PyValueError, _>("Keygen failed!")),
    };
    let sk = PyBswCpAbeSecretKey { sk };
    Ok(sk)
}

/// Decrypt for BSW -> str
///
/// input:
/// - sk: PyBswCpAbeSecretKey
/// - ct: PyBswCiphertext
#[pyfunction]
fn decrypt(sk: &PyBswCpAbeSecretKey, ct: &PyBswCiphertext) -> PyResult<String> {
    let message = match bsw_decrypt(&sk.sk, &ct.ct) {
        Ok(message) => message,
        Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
    };
    let message = match String::from_utf8(message) {
        Ok(message) => message,
        Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
    };
    Ok(message)
}

/// Adds functions that are supposed to be called from python.
/// A function that adds the classes and functions that should be accessible from python.
#[pymodule]
pub fn bsw(_py: Python, m: &PyModule) -> PyResult<()> {
    crate::add_functions!(m;setup,keygen,encrypt,decrypt);
    crate::add_types!(m;PyBswCpAbePublicKey, PyBswCpAbeMasterKey, PyBswCpAbeSecretKey, PyBswCiphertext);
    Ok(())
}