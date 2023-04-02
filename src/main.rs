mod archivo;
mod errors;
mod juego;
use errors::TypeError;

fn main() {
    let tablero = procesar_lectura();
    let mut juego_de_ajedrez = juego::Juego::new(tablero);
    juego_de_ajedrez = analizar_y_cargar_piezas(juego_de_ajedrez);
    let resultado_ataques = juego_de_ajedrez.analisis_de_ataques();
    juego_de_ajedrez.reportar_resultado(resultado_ataques);
}

/// Extrae el nombre del archivo ingresado por consola.
/// Lee contenido del archivo.
/// Retorna una matriz de chars de 8 x 8,
/// con los caracteres ingresados por archivo.
pub fn procesar_lectura() -> [[char; 8]; 8] {
    let mut archivo = archivo::FileHandler::new(std::env::args().collect());
    let resultado_lectura = archivo.leer();
    main_catch(resultado_lectura);
    archivo.dar_tablero_procesado()
}
/// Recibe una variable mutable de tipo Juego, con el tablero precargado.
/// Carga las piezas blanca y negra en la variable Juego.
pub fn analizar_y_cargar_piezas(mut juego_de_ajedrez: juego::Juego) -> juego::Juego {
    let analisis_piezas = juego_de_ajedrez.definir_piezas_en_tablero();
    main_catch(analisis_piezas);
    juego_de_ajedrez
}
/// Catch generico utilizado para los errores específicos de este programa
pub fn main_catch(handler: Result<(), TypeError>) {
    if let Err(handler_error) = handler {
        errors::catch(handler_error)
    }
}
