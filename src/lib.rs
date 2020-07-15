use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn ejecutar_comando_remoto() -> PyResult<&'static str> {
    Ok("Rustecedo es credo")
}

#[pymodule]
fn paqueteando(_py: Python, m:&PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(ejecutar_comando_remoto))?;
    Ok(())
}
