use crate::juego::piezas::AnalisisAtaque;
use crate::juego::piezas::NuevaPiezaAjedrez;


#[derive(Debug)]
pub struct Rey{
    fila: usize,
    columna: usize,
}

impl Rey{
    pub fn new(f: usize, c: usize) -> Self{
        Rey{fila: f, columna: c}
    }
}

impl AnalisisAtaque for Rey{

    fn puedo_atacar_enemigo(&self, pieza_contrincante: NuevaPiezaAjedrez<()>) -> bool{
        println!("{} {} y enemigo en {:?}",self.fila,self.columna,pieza_contrincante.tipo);
        true
    }
}