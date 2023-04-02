extern crate rust_tp1_carniglia;
use rust_tp1_carniglia::{errors, juego::Juego};

#[test]
fn prueba_integracion_ganan_negras() {
    let juego = crear_juego_con_piezas_en('t', 5, 6, 'D', 2, 3);
    assert_eq!(juego.analisis_de_ataques(), (false, true));
}

#[test]
fn prueba_integracion_ganan_blancas() {
    let juego = crear_juego_con_piezas_en('a', 6, 2, 'P', 2, 6);
    assert_eq!(juego.analisis_de_ataques(), (true, false));
}

#[test]
fn prueba_integracion_empate() {
    let juego = crear_juego_con_piezas_en('t', 3, 2, 'R', 2, 2);
    assert_eq!(juego.analisis_de_ataques(), (true, true));
}

#[test]
fn prueba_integracion_todos_pierden() {
    let juego = crear_juego_con_piezas_en('d', 3, 2, 'P', 1, 5);
    assert_eq!(juego.analisis_de_ataques(), (false, false));
}

fn crear_juego_con_piezas_en(
    char_pieza_blanca: char,
    fil_pieza_blanca: usize,
    col_pieza_blanca: usize,
    char_pieza_negra: char,
    fil_pieza_negra: usize,
    col_pieza_negra: usize,
) -> Juego {
    let mut tablero = tablero_vacio();

    tablero[fil_pieza_blanca][col_pieza_blanca] = char_pieza_blanca;
    tablero[fil_pieza_negra][col_pieza_negra] = char_pieza_negra;
    let mut juego_de_ajedrez = Juego::new(tablero);
    let analisis_piezas = juego_de_ajedrez.definir_piezas_en_tablero();
    if let Err(handler_error) = analisis_piezas {
        errors::catch(handler_error)
    }
    juego_de_ajedrez
}

fn tablero_vacio() -> [[char; 8]; 8] {
    [
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
    ]
}
