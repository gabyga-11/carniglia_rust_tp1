use std::process::exit;

#[derive(Debug)]
pub enum TypeError{
    //ErrorPath,
    NombreDeArchivoInvalido,
    //ErrorArgs,
    AperturaDeArchivoInvalida,
    TamanioDeTableroIncorrecto,
    ArchivoConFormatoDeEspaciosIncorrecta,
    ArchivoConCantidadDeCasillerosVaciosIncorrecta,
}

pub fn catch(tipo: TypeError){
    println!("ERROR: {:?}",tipo);
    exit(-1);
}