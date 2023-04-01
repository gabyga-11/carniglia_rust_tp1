use crate::juego::piezas::AnalisisAtaque;
use crate::juego::piezas::PiezaAjedrez;
use super::Color;



#[derive(Debug)]
pub struct Peon{
    fila: usize,
    columna: usize,
    color: Color,
} 

impl Peon{
    pub fn new(f: usize, c: usize, color: Color) -> Self{
        Peon{fila: f, columna: c, color: color}
    }
}

impl AnalisisAtaque for Peon{

    fn puedo_atacar_enemigo(&self, pieza_contrincante: PiezaAjedrez) -> bool{
        println!("{} {}",self.fila,self.columna);
        true
    }
}