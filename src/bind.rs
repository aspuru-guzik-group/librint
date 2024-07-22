use pyo3::prelude::*;

use crate::gen;
use crate::scf;

#[pymodule]
fn librint(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(gen::generate_numbers, m)?)?;
    m.add_function(wrap_pyfunction!(gen::process_vector, m)?)?;

    m.add_function(wrap_pyfunction!(scf::integral1e, m)?)?;
    m.add_function(wrap_pyfunction!(scf::integral2e, m)?)?;
    m.add_function(wrap_pyfunction!(scf::RHF, m)?)?;
    m.add_function(wrap_pyfunction!(scf::energy, m)?)?;
    Ok(())
}