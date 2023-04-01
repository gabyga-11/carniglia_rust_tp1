use crate::juego::piezas::AnalisisAtaque;

#[derive(Debug,Clone,Copy)]
pub struct Rey{
    fila: i16,
    columna: i16,
}

impl Rey{
    pub fn new(f: i16, c: i16) -> Self{
        Rey{fila: f, columna: c}
    }
}

impl AnalisisAtaque for Rey{

    fn puedo_atacar_enemigo(&self, posicion_contrincante: (i16, i16)) -> bool{
        //Valid King move, if the piece moves from (fila, columna) to (cont.0, cont.1), 
        //the move is valid if and only if |cont.0-fila|<=1 and |cont.1-columna|<=1.
        (self.fila - posicion_contrincante.0).abs() <= 1 && (self.columna - posicion_contrincante.1).abs() <= 1
    }
    fn dar_posicion(self) -> (i16, i16) {
        (self.fila, self.columna)
    }
}