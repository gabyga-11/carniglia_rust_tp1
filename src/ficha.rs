enum TipoFicha{
    Rey,
    Dama,
    Alfil,
    Caballo,
    Torre,
    Peon,
}

pub fn funcion_prueba(){

    let dato = Some(TipoFicha::Alfil);
    println!("salio de funcion_prueba {:?}",dato.is_some());
}

//pub struct Posicion(pub i32,pub i32);