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
}

pub fn catch(tipo: TypeError){
    let mensaje_error = obtener_mensaje_personalizado(tipo);
    println!("ERROR: {}",mensaje_error);
    exit(-1);
}

fn obtener_mensaje_personalizado(tipo: TypeError) -> String{
    match tipo {
        TypeError::NombreDeArchivoInvalido => String::from("Nombre de Archivo Inválido"),
        TypeError::NoSeIngresoArgumentoValidoPorConsola => String::from("No se ha ingresado un argumento válido por consola"),
        TypeError::AperturaDeArchivoInvalida => String::from("Apertura de archivo inválida"),
        TypeError::TamanioDeTableroIncorrecto => String::from("El tamaño del tablero no es correcto"),
        TypeError::ArchivoConFormatoDeEspaciosIncorrecta => String::from("El archivo no respeta el formato correcto"),
        TypeError::ArchivoConCantidadDeCasillerosVaciosIncorrecta => String::from("Cantidad de casilleros vacíos incorrecta"),
        TypeError::ArchivoConFormatoDeEspaciosImparesIncorrecta => String::from("El archivo no respeta la estructura de espacios entre casillas '_' "),
        TypeError::CantidadDePiezasIncorrecta => String::from("La cantidad de piezas en el archivo no es la correcta"),
        TypeError::PiezaInexistenteEnAjedrez => String::from("Se ha ingresado una pieza inexistente en el archivo"),
        TypeError::HayDosPiezasBlancas => String::from("Hay dos piezas blancas en el tablero"),
        TypeError::HayDosPiezasNegras => String::from("Hay dos piezas negras en el tablero"),
    }
}