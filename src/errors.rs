use std::process::exit;

#[derive(Debug)]
pub enum TypeError{
    //ErrorPath,
    ErrorNombreDeArchivoInvalido,
    //ErrorArgs,
    ErrorDeAperturaDeArchivo,
}

pub fn catch(tipo: TypeError){
    println!("ERROR: {:?}",tipo);
    exit(-1);
}