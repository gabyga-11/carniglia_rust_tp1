mod ficha;
mod archivo;
mod errors;
use ficha::funcion_prueba;

fn main() {
    println!("Hello, mundo!");
    funcion_prueba();

    //let dondeta = ficha::Posicion(3,4);
    //println!("{:?}",dondeta.0);
    //println!("{:?}",file!());

    let archivo = archivo::FileHandler::new(std::env::args().collect());

    let resultado_lectura = archivo.leer();
    match resultado_lectura {
        Ok(linea) => linea,
        Err(resultado_errores) => errors::catch(resultado_errores)
    }

    //si no hay args[1], cortar ejecucion

    
    //let path = format!("{}{}","src/".to_owned(),args[1]);
    //println!("{:?}",path);



    //let apertura_archivo = File::open(path);
    //handlear error


    //let mut archivo = match apertura_archivo {
    //    Ok(archivo) => archivo,
    //    Err(error) => panic!("ERROR: {:?}", error), //TODO: No usar panic
    //};

    //let mut buffer = String::new();
    
    //let mut contenido = archivo.read_to_string(&mut buffer);
    
    
    //let mut resultado = archivo::leer_linea_por_linea(args[1].as_str());

    //match resultado {
        //Ok(linea) => linea,
        //Err(error) => panic!("ERROR: {:?}", error), //TODO: No usar panic
    //};


}





/* 
pub fn generic_match(resultadoHandler: Result<T,E>) -> T {
    match resultadoHandler {
        Ok(resultado) => resultado,
        Err(resultado_errores) => errors::catch(resultado_errores)
    }
}*/