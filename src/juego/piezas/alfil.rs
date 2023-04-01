use crate::juego::piezas::AnalisisAtaque;
use crate::juego::piezas::PiezaAjedrez;


#[derive(Debug)]
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

    fn puedo_atacar_enemigo(&self, pieza_contrincante: PiezaAjedrez) -> bool{
        println!("{} {}",self.fila,self.columna);
        true
    }
}