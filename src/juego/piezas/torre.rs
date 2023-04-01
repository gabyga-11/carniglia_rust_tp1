use crate::juego::piezas::AnalisisAtaque;
use crate::juego::piezas::NuevaPiezaAjedrez;


#[derive(Debug)]
pub struct Torre{
    fila: usize,
    columna: usize,
}

impl Torre{
    pub fn new(f: usize, c: usize) -> Self{
        Torre{fila: f, columna: c}
    }
}
/* 
impl AnalisisAtaque for Torre{

    fn puedo_atacar_enemigo(&self, pieza_contrincante: PiezaAjedrez) -> bool{
        println!("{} {}",self.fila,self.columna);
        true
    }
}*/