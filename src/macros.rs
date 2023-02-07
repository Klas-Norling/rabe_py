//! Some helper macros for the crate

/// Shorthand for implementing serialization functionality for a type
/// ---
///
/// Implements a `__init__` function that takes a JSON string and returns an instance of the object,
/// also implements a `__str__` function that takes the object and returns a JSON representation of the object.
#[macro_export]
macro_rules! serializable {
    ($($id:ident),+) => {
        paste::paste!{
            $(

                // Implement
                #[pymethods]
                impl $id {
                    #[new]
                    #[doc = "Creates a new instance of [`" [<$id>] "`] from a given JSON string."]
                    pub fn __init__(value: String) -> PyResult<Self> {
                        match serde_json::from_str(&value) {
                            Ok(value) => Ok(value),
                            Err(e) =>  Err(PyErr::new::<PyValueError, _>(format!("{}",e)))
                        }
                    }
                    #[doc = "Creates a JSON string representation from a [`" [<$id>] "`]."]
                    pub fn __str__(&self) -> PyResult<String> {
                        match serde_json::to_string(&self) {
                            Ok(value) => Ok(value),
                            Err(e) => Err(PyErr::new::<PyValueError, _>(format!("{}", e)))
                        }
                    }
                }
            )+
        }
    };
}
/// Shorthand for adding a bunch of functions to a python module
#[macro_export]
macro_rules! add_functions {
    ($m:ident; $($function:ident),+) => {
      $(
        $m.add_function(wrap_pyfunction!($function, $m)?)?;
      )+
    };
}
/// Shorthand for adding a bunch of types to a python module
#[macro_export]
macro_rules! add_types {
    ($m:ident; $($t:ident),+) => {
      $(
        $m.add_class::<$t>()?;
      )+
    };
}