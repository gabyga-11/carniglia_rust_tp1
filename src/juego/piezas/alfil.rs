use crate::juego::piezas::AnalisisAtaque;
use crate::juego::piezas::PiezaAjedrez;


#[derive(Debug,Clone,Copy)]
pub struct Alfil{
    fila: usize,
    columna: usize,
}

impl Alfil{
    pub fn new(f: usize, c: usize) -> Self{
        Alfil{fila: f, columna: c}
    }
}

impl AnalisisAtaque for Alfil{

    fn puedo_atacar_enemigo(&self, posicion_contrincante: (usize, usize)) -> bool{
        println!("{} {}",self.fila,self.columna);
        println!("{} {}",posicion_contrincante.0,posicion_contrincante.1);
        true
    }
    fn dar_posicion(self) -> (usize, usize) {
        (self.fila, self.columna)
    }
}