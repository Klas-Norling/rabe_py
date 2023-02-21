


use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use rabe::schemes::bdabe::{
    setup as bdabe_setup, authgen as bdabe_authgen, keygen as bdabe_keygen,
    request_attribute_pk as bdabe_request_attribute_pk, request_attribute_sk as bdabe_request_attribute_sk,
    encrypt as bdabe_encrypt, decrypt as bdabe_decrypt,
};

use rabe::utils::policy::pest::PolicyLanguage;

pub mod types;
use types::*;

#[pyfunction]
pub fn setup() -> PyResult<(PyBdabePublicKey, PyBdabeMasterKey)> {
    let (pk, mk) = bdabe_setup();
    let pk = PyBdabePublicKey { pk };
    let msk = PyBdabeMasterKey { msk };
    Ok((pk, mk))
}

#[pyfunction]
pub fn authgen(
    pk: &PyBdabePublicKey,
    mk: &PyBdabeMasterKey,
    name: String
) -> PyResult<PyBdabeSecretAuthorityKey> {
    let sak: PyBdabeSecretAuthorityKey = match bdabe_authgen(&pk.pk, &mk.mk, &name) {
        Ok(sak) => PyBdabeSecretAuthorityKey{ sak },
        Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
    };
    Ok(sak)
}

pub fn request_attribute_pk(
    pk: &PyBdabePublicKey,
    sak: &PyBdabeSecretAuthorityKey,
    attribute: String
) -> PyResult<PyBdabePublicAttributeKey> {
    let pak: PyBdabePublicAttributeKey = match bdabe_request_attribute_pk(&pk.pk, &sak.sak, &attribute) {
        Ok(pak) => PyBdabePublicAttributeKey{ pak },
        Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
    };
    Ok(pak)
}

pub fn encrypt(
    pk: &PyBdabePublicKey,
    attr_pak: &Vec<PyBdabePublicAttributeKey>,
    policy: String,
    plaintext: String,
) -> PyResult<PyBdabeCiphertext> {
    let plaintext = plaintext.into_bytes();
    let ct: PyBdabeCiphertext = match bdabe_encrypt(&pk.pk, &attr_pak, &policy, &plaintext, PolicyLanguage::HumanPolicy) {
        Ok(ct) => PyBdabeCiphertext{ ct },
        Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
    };
    Ok(ct)
}

pub fn decrypt(
    pk: &PyBdabePublicKey,
    uk: &PyBdabeUserKey,
    ct: &PyBdabeCiphertext,
) -> PyResult<Vec<u8>> {
    let plaintext: Vec<u8> = match bdabe_decrypt(&pk.pk, &uk.uk, &ct.ct) {
        Ok(plaintext) => plaitnext,
        Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
    };
    Ok(plaintext)
}




#[pymodule]
pub fn bdabe(_py: Python, m: &PyModule) -> PyResult<()> {
    crate::add_functions!(m;setup, decrypt, encrypt, authgen, authgen, request_attribute_pk);
    crate::add_types!(m;BdabePublicKey, PyBdabeMasterKey, PyBdabeCiphertext PyPolicyLanguage, PyBdabePublicKey, PyBdabePublicUserKey, PyBdabeSecretAuthorityKey, PyBdabePublicAttributeKey);
    Ok(())
}