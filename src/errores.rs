use std;
use std::error::Error;
use pyo3::{exceptions,PyErr};

pub type BoxError = Box<dyn Error + std::marker::Send + std::marker::Sync>;

pub struct ConexionError {
    pub mensaje: String,
    pub source: Option<BoxError>
}

impl ConexionError {
    pub fn mensaje(&self) -> String {
        self.mensaje.clone()
    }
}

impl std::fmt::Display for ConexionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ocurrió un error al conectarse {}", self.mensaje)?;
        if let Some(error) = &self.source {
            write!(f, "\nOriginado por: {}", error)?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for ConexionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <ConexionError as std::fmt::Display>::fmt(self, f)
    }
}

impl std::convert::From<ConexionError> for PyErr {
    fn from(err: ConexionError) -> PyErr {
        exceptions::OSError::py_err(err.to_string())
    }
}

impl Error for ConexionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.source {
            Some(error) => Some(error.as_ref()),
            None => None
        }
    }
}
