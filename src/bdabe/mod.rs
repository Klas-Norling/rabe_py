


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
    let pak: PyBdabePublicAttributeKey = match bdabe_request_attribute_pk(&pk.pj, &sak.sak, &attribute) {
        Ok(pak) => PyBdabePublicAttributeKey{ pak },
        Err(e) => return Err(PyErr::new::<PyValueError, _>(format!("{}", e))),
    };
    Ok(pak)
}



#[pymodule]
pub fn bdabe(_py: Python, m: &PyModule) -> PyResult<()> {
    crate::add_functions!(m;setup, decrypt, encrypt, keygen, authgen, request_attribute_pk, request_attribute_sk);
    crate::add_types!(m;BdabePublicKey, PyBdabeMasterKey, PyBdabeCiphertext PyPolicyLanguage);
    Ok(())
}