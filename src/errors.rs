use std::process::exit;

#[derive(Debug)]
pub enum TypeError{
    //ErrorPath,
    NombreDeArchivoInvalido,
    NoSeIngresoArgumentoValidoPorConsola,
    AperturaDeArchivoInvalida,
    TamanioDeTableroIncorrecto,
    ArchivoConFormatoDeEspaciosIncorrecta,
    ArchivoConCantidadDeCasillerosVaciosIncorrecta,
    ArchivoConFormatoDeEspaciosImparesIncorrecta,
    CantidadDePiezasIncorrecta,
    PiezaInexistenteEnAjedrez,
    HayDosPiezasBlancas,
    HayDosPiezasNegras,
}

pub fn catch(tipo: TypeError){
    println!("ERROR: {:?}",tipo);
    exit(-1);
}