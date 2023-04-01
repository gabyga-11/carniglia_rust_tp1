use super::AnalisisAtaque;

#[derive(Debug, Clone, Copy)]
pub struct Dama {
    fila: i16,
    columna: i16,
}

impl Dama {
    pub fn new(f: i16, c: i16) -> Self {
        Dama {
            fila: f,
            columna: c,
        }
    }
}

impl AnalisisAtaque for Dama {
    fn puedo_atacar_enemigo(&self, posicion_contrincante: (i16, i16)) -> bool {
        //Valid Queen move, a queen's move is valid if it is either a valid bishop or rook move.
        (self.fila == posicion_contrincante.0 || self.columna == posicion_contrincante.1)
            || ((self.fila - posicion_contrincante.0).abs()
                == (self.columna - posicion_contrincante.1).abs())
    }

    fn dar_posicion(self) -> (i16, i16) {
        (self.fila, self.columna)
    }
}
