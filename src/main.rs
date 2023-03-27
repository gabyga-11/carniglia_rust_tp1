mod ficha;
use ficha::funcion_prueba;
use std::fs::File;
use std::io::prelude::*;
fn main() {
    println!("Hello, mundo!");
    funcion_prueba();

    let dondeta = ficha::Posicion(3,4);
    println!("{:?}",dondeta.0);
    println!("{:?}",file!());

    let args: Vec<String> = std::env::args().collect();
    println!("{:?}",args);

    let path = format!("{}{}","src/".to_owned(),args[1]);






    let apertura_archivo = File::open(path);
    //handlear error


    let mut archivo = match apertura_archivo {
        Ok(archivo) => archivo,
        Err(error) => panic!("ERROR: {:?}", error), //TODO: No usar panic
    };

    let mut contenido = String::new();
    
    let contenido = archivo.read_to_string(&mut contenido);


    println!("{:?}",contenido);
}
