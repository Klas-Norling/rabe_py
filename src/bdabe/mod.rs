//! BDABE Encryption and Decryption interface for python
//! ---
//! This is a wrapper
//! Implented in [the aw11 source](rabe::schemes::bdabe)

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use rabe::schemes::bdabe::{
    authgen as bdabe_authgen, decrypt as bdabe_decrypt, encrypt as bdabe_encrypt,
    keygen as bdabe_keygen, request_attribute_pk as bdabe_request_attribute_pk,
    request_attribute_sk as bdabe_request_attribute_sk, setup as bdabe_setup,
};

use rabe::utils::policy::pest::PolicyLanguage;

pub mod types;
use types::*;


/// The setup algorithm of BDABE. Generates a PyBdabePublicKey and a PyBdabeMasterKey.
#[pyfunction]
pub fn setup() -> PyResult<(PyBdabePublicKey, PyBdabeMasterKey)> {
    let (pk, mk) = bdabe_setup();
    let pk = PyBdabePublicKey { pk };
    let mk = PyBdabeMasterKey { mk };
    Ok((pk, mk))
}

/// Sets up and generates a new Authority by creating a secret authority key (SKauth).
/// The key is created for an authority with a given "name".
///
/// # Arguments
///
///	* `pk` - A PyBdabePublicKey (PK), generated by setup()
///	* `mk` - A PyBdabeMasterKey (MK), generated by setup()
///	* `name` - The name of the authority the key is associated with. Must be unique.
#[pyfunction]
pub fn authgen(
    pk: &PyBdabePublicKey,
    mk: &PyBdabeMasterKey,
    name: String,
) -> PyResult<PyBdabeSecretAuthorityKey> {
    let ag = bdabe_authgen(&pk.pk, &mk.mk, &name);
    let sak = PyBdabeSecretAuthorityKey { sak: ag };
    Ok(sak)
}

/// Sets up and generates a new User by creating a secret user key (uk).
/// The key is created for an user with a given "name".
/// It consists of a BdabeSecretUserKey and a BdabePublicUserKey as well as
/// an empty vector of BdabeSecretAttributeKeys.
///
/// # Arguments
///
///	* `pk` - A PyBdabePublicKey (PK), generated by setup()
///	* `sak` - A PyBdabeSecretAuthorityKey (sak), associated with an authority and generated by authgen()
///	* `name` - The name of the user the key is associated with. Must be unique.
#[pyfunction]
pub fn keygen(
    pk: &PyBdabePublicKey,
    sak: &PyBdabeSecretAuthorityKey,
    name: String,
) -> PyResult<PyBdabeUserKey> {
    let temp = bdabe_keygen(&pk.pk, &sak.sak, &name);
    let uk = PyBdabeUserKey { uk: temp };
    Ok(uk)
}

/// Some doc comment
#[pyfunction]
pub fn request_attribute_pk(
    pk: &PyBdabePublicKey,
    sak: &PyBdabeSecretAuthorityKey,
    attribute: String,
) -> PyResult<PyBdabePublicAttributeKey> {
    let pak: PyBdabePublicAttributeKey =
        match bdabe_request_attribute_pk(&pk.pk, &sak.sak, &attribute) {
            Ok(pak) => PyBdabePublicAttributeKey { pak: vec![pak] },
            Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
        };
    Ok(pak)
}
/// Some doc comment
#[pyfunction]
pub fn request_attribute_sk(
    _pku: &PyBdabeUserKey,
    _ska: &PyBdabeSecretAuthorityKey,
    _attribute: String,
) -> PyResult<PyBdabeSecretAttributeKey> {
    let sak = match bdabe_request_attribute_sk(&_pku.uk._pk, &_ska.sak, &_attribute) {
        Ok(sak) => PyBdabeSecretAttributeKey { sak },
        Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
    };
    Ok(sak)
}

/// Some doc comment
#[pyfunction]
pub fn encrypt(
    pk: &PyBdabePublicKey,
    attr_pak: &PyBdabePublicAttributeKey,
    policy: String,
    plaintext: String,
) -> PyResult<PyBdabeCiphertext> {
    let plaintext = plaintext.into_bytes();
    let ct: PyBdabeCiphertext = match bdabe_encrypt(
        &pk.pk,
        &attr_pak.pak,
        &policy,
        &plaintext,
        PolicyLanguage::HumanPolicy,
    ) {
        Ok(ct) => PyBdabeCiphertext { ct },
        Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
    };
    Ok(ct)
}

/// Some doc comment
#[pyfunction]
pub fn decrypt(
    pk: &PyBdabePublicKey,
    uk: &PyBdabeUserKey,
    ct: &PyBdabeCiphertext,
) -> PyResult<Vec<u8>> {
    let plaintext: Vec<u8> = match bdabe_decrypt(&pk.pk, &uk.uk, &ct.ct) {
        Ok(plaintext) => plaintext,
        Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
    };
    Ok(plaintext)
}

/// Adds functions that are supposed to be called from python.
/// A function that adds the classes and functions that should be accessible from python.
#[pymodule]
pub fn bdabe(_py: Python, m: &PyModule) -> PyResult<()> {
    crate::add_functions!(m;setup,authgen,keygen,request_attribute_pk,encrypt,decrypt,request_attribute_sk);
    crate::add_types!(m;PyBdabePublicKey, PyBdabeMasterKey, PyBdabeCiphertext, PyBdabePublicKey, PyBdabePublicUserKey, PyBdabeSecretAuthorityKey, PyBdabePublicAttributeKey);
    Ok(())
}