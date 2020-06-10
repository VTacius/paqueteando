use std::net::TcpStream;
use std::net::SocketAddrV4;
use std::str::FromStr;
use ssh2::Session;
use std::error::Error;
use pyo3::{exceptions, PyErr, PyResult};

pub type BoxError = Box<dyn Error + std::marker::Send + std::marker::Sync>;

pub struct ConexionError {
    mensaje: String,
    source: Option<BoxError>
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

fn conexion() -> Result<Session, BoxError>{
    let servidor = "127.0.0.abecedario";
    let usuario = "root";
    let socket = SocketAddrV4::from_str(servidor)?;
    let tcp = TcpStream::connect(socket.to_string())?;
    let mut sesion = Session::new()?;

    sesion.set_tcp_stream(tcp);
    sesion.set_compress(true);
    sesion.handshake()?;
    sesion.userauth_agent(usuario)?;

    if sesion.authenticated() {
        Ok(sesion)
    } else {
        Err("No se pudo autenticar, lo siento".into())
    }
}

pub fn conectar() -> Result<Session, ConexionError>{
    match conexion() {
        Ok(sesion) => Ok(sesion),
        Err(error) => Err(ConexionError{mensaje: String::from("Nada, acá fallando"), source: Some(error)})
    }
}