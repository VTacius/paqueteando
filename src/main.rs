pub mod conexion;
use conexion::conectar;


fn main(){
    let sesion = conectar().unwrap();
}