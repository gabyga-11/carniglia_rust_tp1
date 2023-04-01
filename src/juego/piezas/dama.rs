use crate::juego::piezas::AnalisisAtaque;
use crate::juego::piezas::PiezaAjedrez;


#[derive(Debug,Clone,Copy)]
pub struct Dama{
    fila: usize,
    columna: usize,
}

impl Dama{
    pub fn new(f: usize, c: usize) -> Self{
        Dama{fila: f, columna: c}
    }
}

impl AnalisisAtaque for Dama{
    fn puedo_atacar_enemigo(&self, posicion_contrincante: (usize, usize)) -> bool{
        println!("{} {}",self.fila,self.columna);
        println!("{} {}",posicion_contrincante.0,posicion_contrincante.1);
        true
    }
    
    fn dar_posicion(self) -> (usize, usize) {
        (self.fila, self.columna)
    }
}
