use crate::juego::piezas::AnalisisAtaque;
use crate::juego::piezas::PiezaAjedrez;
use super::Color;



#[derive(Debug,Clone,Copy)]
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

    fn puedo_atacar_enemigo(&self, posicion_contrincante: (usize, usize)) -> bool{
        println!("{} {}",self.fila,self.columna);
        println!("{} {}",posicion_contrincante.0,posicion_contrincante.1);
        true
    }
    fn dar_posicion(self) -> (usize, usize) {
        (self.fila, self.columna)
    }
}