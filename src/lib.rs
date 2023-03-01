//! Code Formatting constrictions
#![deny(missing_docs)]
#![deny(non_snake_case)]
#![deny(non_camel_case_types)]
#![deny(clippy::all)]
#![allow(bare_urls)]
#![deny(warnings)]

use pyo3::{prelude::*, wrap_pymodule};

pub mod ac17;
pub mod aw11;
pub mod lsw;
pub mod bdabe;
pub mod bsw;
mod macros;

/// A Python module implemented in Rust.
#[pymodule]
fn rabe_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(aw11::aw11))?;
    m.add_wrapped(wrap_pymodule!(ac17::ac17))?;
    m.add_wrapped(wrap_pymodule!(lsw::lsw))?;
    m.add_wrapped(wrap_pymodule!(bdabe::bdabe))?;
    m.add_wrapped(wrap_pymodule!(bsw::bsw))?;


    Ok(())
}
