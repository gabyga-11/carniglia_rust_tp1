use std::process::exit;

#[derive(Debug, PartialEq)]
/// Tipos de errores específicos para este proyecto.
/// Los nombres son autoexplicativos.
pub enum TypeError {
    NombreDeArchivoInvalido,
    AperturaDeArchivoInvalida,
    TamanioDeTableroIncorrecto,
    ArchivoConFormatoDeEspaciosIncorrecta,
    ArchivoConCantidadDeCasillerosVaciosIncorrecta,
    ArchivoConFormatoDeEspaciosImparesIncorrecta,
    CantidadDePiezasIncorrecta,
    PiezaInexistenteEnAjedrez,
    HayDosPiezasBlancas,
    HayDosPiezasNegras,
    CantidadDeArgumentosIngresadosIncorrecta,
}

/// En base a un error de los posibles en el enum TypeError,
/// obtiene el mensaje personalizado para cada error
/// y muestra por pantalla el mensaje.
/// Finalmente, sale del programa.
pub fn catch(tipo: TypeError) {
    let mensaje_error = obtener_mensaje_personalizado(tipo);
    println!("ERROR: {}", mensaje_error);
    exit(1);
}
/// En base a un error de los posibles en el enum TypeError,
/// asigna un mensaje personalizado para cada error, y retorna el mismo
fn obtener_mensaje_personalizado(tipo: TypeError) -> String {
    match tipo {
        TypeError::NombreDeArchivoInvalido => String::from("Nombre de Archivo Inválido"),
        TypeError::AperturaDeArchivoInvalida => String::from("Apertura de archivo inválida"),
        TypeError::TamanioDeTableroIncorrecto => {
            String::from("El tamaño del tablero no es correcto")
        }
        TypeError::ArchivoConFormatoDeEspaciosIncorrecta => {
            String::from("El archivo no respeta el formato correcto")
        }
        TypeError::ArchivoConCantidadDeCasillerosVaciosIncorrecta => {
            String::from("Cantidad de casilleros vacíos incorrecta")
        }
        TypeError::ArchivoConFormatoDeEspaciosImparesIncorrecta => {
            String::from("El archivo no respeta la estructura de espacios entre casillas '_' ")
        }
        TypeError::CantidadDePiezasIncorrecta => {
            String::from("La cantidad de piezas en el archivo no es la correcta")
        }
        TypeError::PiezaInexistenteEnAjedrez => {
            String::from("Se ha ingresado una pieza inexistente en el archivo")
        }
        TypeError::HayDosPiezasBlancas => String::from("Hay dos piezas blancas en el tablero"),
        TypeError::HayDosPiezasNegras => String::from("Hay dos piezas negras en el tablero"),
        TypeError::CantidadDeArgumentosIngresadosIncorrecta => {
            String::from("Cantidad de argumentos ingresados por consola inválido")
        }
    }
}
