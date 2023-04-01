use super::AnalisisAtaque;

#[derive(Debug, Clone, Copy)]
pub struct Torre {
    fila: i16,
    columna: i16,
}

impl Torre {
    pub fn new(f: i16, c: i16) -> Self {
        Torre {
            fila: f,
            columna: c,
        }
    }
}

impl AnalisisAtaque for Torre {
    fn puedo_atacar_enemigo(&self, posicion_contrincante: (i16, i16)) -> bool {
        //Valid Rook move, if the piece moves from (X1, Y1) to (X2, Y2), the move is valid if and only
        //if X2=X1 or Y2=Y1.
        self.fila == posicion_contrincante.0 || self.columna == posicion_contrincante.1
    }
    fn dar_posicion(self) -> (i16, i16) {
        (self.fila, self.columna)
    }
}
