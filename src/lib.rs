#![feature(proc_macro, specialization, const_fn)]
extern crate pyo3;

use pyo3::prelude::*;

#[py::modinit(_pyrust_example)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "example_fn")]
    fn search(_py: Python, input_str: String) -> PyResult<i32> {
        Ok(input_str.len() as i32)
    }

    Ok(())
}
