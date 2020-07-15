use std::net::TcpStream;
use std::net::SocketAddrV4;
use std::str::FromStr;
use ssh2::Session;
use std::error::Error;
use pyo3::{exceptions, PyErr, PyResult};

use crate::errores::{BoxError, ConexionError};

pub fn conexion() -> Result<Session, BoxError>{
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