mod errores;
mod conexion;
use conexion::conectar;

use std::error::Error;
use std::boxed::Box;

pub type BoxError = Box<dyn Error + std::marker::Send + std::marker::Sync>;

macro_rules! estructurar {
    ($nombre: ident, $mensaje: expr) => {
        pub struct $nombre {
            pub source: Option<BoxError>
        }

        impl std::fmt::Display for $nombre {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $mensaje)?;
                if let Some(error) = &self.source {
                    write!(f, "\nOriginado por: {}", error)?;
                }
                Ok(())
            }
        }
        impl std::fmt::Debug for $nombre {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
                <$nombre as std::fmt::Display>::fmt(self, f)
            }
        }

        impl std::convert::From<ConexionError> for PyErr {
            fn from(err: ConexionError) -> PyErr {
                exceptions::OSError::py_err(err.to_string())
            }
        }

        impl Error for $nombre {
            fn source(&self) -> Option<&(dyn Error + 'static)> {
                match &self.source {
                    Some(error) => Some(error.as_ref()),
                    None => None
                }
            }
        }
    }
}

estructurar!(ErrorConfiguracion, "Ocurrió un error con la configuración");
estructurar!(ErrorEquiparacion, "Ocurrió un error con la equiparación");
fn main(){
    let source = None;
    let c = ErrorEquiparacion{source};
    println!("{:?}", c);
}