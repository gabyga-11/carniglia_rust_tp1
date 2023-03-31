use crate::juego::piezas::AnalisisAtaque;
use crate::juego::piezas::PiezaAjedrez;
use crate::juego::piezas::Color;
#[derive(Debug)]
pub struct Rey{
    fila: i32,
    columna: i32,
    color: Color
}

impl AnalisisAtaque for Rey{
    fn ver_si_atacar_enemigo(&self, pieza_contrincante: PiezaAjedrez) -> bool{
        true
    }
}