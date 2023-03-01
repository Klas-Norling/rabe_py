//! The python wrapper for ac17
//! ---
//! This is a wrapper
//! Implemented in [the aw11 source](rabe::schemes::ac17)

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use rabe::schemes::ac17::{
    cp_decrypt as ac17_cp_decrypt, cp_encrypt as ac17_cp_encrypt, cp_keygen as ac17_cp_keygen,
    kp_decrypt as ac17_kp_decrypt, kp_encrypt as ac17_kp_encrypt, kp_keygen as ac17_kp_keygen,
    setup as ac17_setup,
};
use rabe::utils::policy::pest::PolicyLanguage;
use std::str;

pub mod types;
use types::*;

/// setup for both AC17CP and AC17KP, will return PyAc17PublicKey and PyAc17MasterKey objects
#[pyfunction]
pub fn setup() -> PyResult<(PyAc17PublicKey, PyAc17MasterKey)> {
    let (pk, msk) = ac17_setup();
    let pk = PyAc17PublicKey { pk };
    let msk = PyAc17MasterKey { msk };
    Ok((pk, msk))
}

/// Encryption function for AC17KP, will return PyAc17KpCiphertext object
/// -----
///
/// ## Inputs:
/// * pk: PyAc17PublicKey (Generated from setup)
/// * attributes: list[str]
/// * plaintext: str
#[pyfunction]
pub fn kp_encrypt(
    pk: &PyAc17PublicKey,
    attributes: Vec<String>,
    plaintext: String,
) -> PyResult<PyAc17KpCiphertext> {
    let plaintext = plaintext.into_bytes();
    let ct = match ac17_kp_encrypt(&pk.pk, &attributes, &plaintext) {
        Ok(ct) => PyAc17KpCiphertext { ct },
        Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
    };
    Ok(ct)
}

/// Decryption function for AC17KP, will return String (python str)
/// -----
///
/// ## Inputs:
/// * sk: PyAc17KpSecretKey (Generated from kp_keygen)
/// * ct: PyAc17KpCiphertext (Generated from kp_encrypt)
#[pyfunction]
pub fn kp_decrypt(sk: &PyAc17KpSecretKey, ct: &PyAc17KpCiphertext) -> PyResult<String> {
    let plaintext: Vec<u8> = match ac17_kp_decrypt(&sk.sk, &ct.ct) {
        Ok(plaintext) => plaintext,
        Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
    };
    let plaintext = match str::from_utf8(&plaintext) {
        Ok(plaintext) => String::from(plaintext),
        Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
    };

    Ok(plaintext)
}

/// Key generation function for AC17KP, will return PyAc17KpSecretKey
/// -----
///
/// ## Inputs:
/// * msk: PyAc17MasterKey (Generated from setup)
/// * policy: str (Ex: '"A" and "B"')
#[pyfunction]
pub fn kp_keygen(msk: &PyAc17MasterKey, policy: String) -> PyResult<PyAc17KpSecretKey> {
    // FIXME: HumanPolicy should not be the only default
    let sk = match ac17_kp_keygen(&msk.msk, &policy, PolicyLanguage::HumanPolicy) {
        Ok(sk) => PyAc17KpSecretKey { sk },
        Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
    };
    Ok(sk)
}

/// Encryption function for Ac17Cp, will return PyAx17CpCiphertext object
/// -----
///
/// ## Inputs:
/// * pk: PyAc17PublicKey (Generated from setup)
/// * policy: str
/// * plaintext: str
#[pyfunction]
pub fn cp_encrypt(
    pk: &PyAc17PublicKey,
    policy: String,
    plaintext: String,
) -> PyResult<PyAc17CpCiphertext> {
    let plaintext = plaintext.as_bytes();
    let ct = match ac17_cp_encrypt(&pk.pk, &policy, &plaintext, PolicyLanguage::HumanPolicy) {
        Ok(ct) => PyAc17CpCiphertext { ct },
        Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
    };
    Ok(ct)
}

/// Key generation function for AC17KP, will return PyAc17KpSecretKey
/// -----
///
/// ## Inputs:
/// * msk: PyAc17MasterKey (Generated from setup)
/// * attributes: list[str]
#[pyfunction]
pub fn cp_keygen(msk: &PyAc17MasterKey, attributes: Vec<String>) -> PyResult<PyAc17CpSecretKey> {
    let sk = match ac17_cp_keygen(&msk.msk, &attributes) {
        Ok(sk) => PyAc17CpSecretKey { sk },
        Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
    };
    Ok(sk)
}

/// Decryption function for AC17CP, will return String (python str).
/// -----
///
/// ## Inputs:
/// * sk: PyAc17CpSecretKey (Generated from cp_keygen)
/// * ct: PyAc17CpCiphertext (Generated from cp_encrypt)
#[pyfunction]
pub fn cp_decrypt(sk: &PyAc17CpSecretKey, ct: &PyAc17CpCiphertext) -> PyResult<String> {
    let plaintext = match ac17_cp_decrypt(&sk.sk, &ct.ct) {
        Ok(plaintext) => plaintext,
        Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
    };
    let plaintext = match str::from_utf8(&plaintext) {
        Ok(plaintext) => String::from(plaintext),
        Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
    };

    Ok(plaintext)
}

/// * Adds functions that are supposed to be called from python.
/// * A function that adds the classes and functions that should be accessible from python.
#[pymodule]
pub fn ac17(_py: Python, m: &PyModule) -> PyResult<()> {
    crate::add_functions!(m;setup, cp_decrypt, cp_encrypt, cp_keygen, kp_decrypt, kp_encrypt, kp_keygen);
    crate::add_types!(m;PyAc17CpCiphertext, PyAc17CpSecretKey, PyAc17KpCiphertext, PyAc17KpSecretKey, PyAc17MasterKey, PyAc17PublicKey);
    Ok(())
}
