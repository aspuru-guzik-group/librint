use pyo3::prelude::*;

#[pyfunction]
pub fn process_vector(mut vec: Vec<f64>) -> PyResult<Vec<f64>> {
    for elem in vec.iter_mut() {
        *elem *= 2.0; // Example operation: doubling each element
    }

    Ok(vec)
}

#[pyfunction]
pub fn generate_numbers() -> PyResult<Vec<f64>> {
    Ok(vec![1.0, 2.0, 3.0, 4.0, 5.0])
}