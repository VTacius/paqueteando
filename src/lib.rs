use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn espacio() -> PyResult<&'static str> {
    Ok("Rustecedo")
}

#[pymodule]
fn paqueteando(_py: Python, m:&PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(espacio))?;
    Ok(())
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
