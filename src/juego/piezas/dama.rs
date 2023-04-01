use crate::juego::piezas::AnalisisAtaque;
use crate::juego::piezas::PiezaAjedrez;


#[derive(Debug)]
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
    fn puedo_atacar_enemigo(&self, pieza_contrincante: PiezaAjedrez) -> bool{

        println!("{} {}",self.fila,self.columna);
        true
    }
}
