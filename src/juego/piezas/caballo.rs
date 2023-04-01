use crate::juego::piezas::AnalisisAtaque;
use crate::juego::piezas::NuevaPiezaAjedrez;


#[derive(Debug)]
pub struct Caballo{
    fila: usize,
    columna: usize,
}

impl Caballo{
    pub fn new(f: usize, c: usize) -> Self{
        Caballo{fila: f, columna: c}
    }
}
/* 
impl AnalisisAtaque for Caballo{

    fn puedo_atacar_enemigo(&self, pieza_contrincante: PiezaAjedrez) -> bool{
        println!("{} {}",self.fila,self.columna);
        true
    }
}*/