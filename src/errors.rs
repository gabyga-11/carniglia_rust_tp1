use std::process::exit;

#[derive(Debug)]
pub enum TypeError{
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
    PiezaInexistenteEnAjedrezCargado,
}

//TODO: Implementar match con mensaje mas amigable

pub fn catch(tipo: TypeError){
    println!("ERROR: {:?}",tipo);
    exit(-1);
}