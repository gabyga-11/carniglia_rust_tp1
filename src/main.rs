mod archivo;
mod errors;
mod juego;
use errors::TypeError;



fn main() {
    println!("Hello, mundo!");
    

    //let dondeta = ficha::Posicion(3,4);
    //println!("{:?}",dondeta.0);
    //println!("{:?}",file!());

    let tablero = procesar_lectura();
    let mut juego_de_ajedrez = juego::Juego::new(tablero);
    juego_de_ajedrez = analizar_y_cargar_piezas(juego_de_ajedrez);
    let resultado_ataques = juego_de_ajedrez.analisis_de_ataques();
    juego_de_ajedrez.reportar_resultado(resultado_ataques);

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


pub fn procesar_lectura() -> [[char; 8]; 8]{
    let mut archivo = archivo::FileHandler::new(std::env::args().collect());

    let resultado_lectura = archivo.leer();
    main_match(resultado_lectura);

    archivo.dar_tablero_procesado()
}

pub fn analizar_y_cargar_piezas(mut juego_de_ajedrez: juego::Juego) -> juego::Juego{
    let analisis_piezas = juego_de_ajedrez.definir_piezas_en_tablero();
    main_match(analisis_piezas);
    juego_de_ajedrez
}

pub fn main_match(handler: Result<(), TypeError>){
    if let Err(handler_error) = handler { errors::catch(handler_error) }
}
/* 
pub fn generic_match(resultadoHandler: Result<T,E>) -> T {
    match resultadoHandler {
        Ok(resultado) => resultado,
        Err(resultado_errores) => errors::catch(resultado_errores)
    }
}*/