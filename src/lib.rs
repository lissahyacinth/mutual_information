use pyo3::prelude::*;

mod mutual_info;
use mutual_info::mutual_information_internal;

#[pyfunction]
/// Calculate Mutual Information for Two paired Vectors
///
/// Will fail if the two vectors are not the same length.
fn mutual_information(a: Vec<f64>, b: Vec<f64>) -> f64 {
    let a_len = a.len();
    assert!(a_len == b.len());
    mutual_information_internal(a, a_len as i32, b, a_len as i32)
}

#[pymodule]
fn vector_ops(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(mutual_information))?;
    Ok(())
}
