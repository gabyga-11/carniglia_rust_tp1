use super::AnalisisAtaque;

#[derive(Debug, Clone, Copy)]
pub struct Caballo {
    fila: i16,
    columna: i16,
}

impl Caballo {
    pub fn new(f: i16, c: i16) -> Self {
        Caballo {
            fila: f,
            columna: c,
        }
    }
}

impl AnalisisAtaque for Caballo {
    fn puedo_atacar_enemigo(&self, posicion_contrincante: (i16, i16)) -> bool {
        //Valid Knight move, if the piece moves from (X1, Y1) to (X2, Y2),
        //the move is valid if and only if (|X2-X1|=1 and |Y2-Y1|=2) or (|X2-X1|=2 and |Y2-Y1|=1).
        ((posicion_contrincante.0 - self.fila).abs() == 1
            && (posicion_contrincante.1 - self.columna).abs() == 2)
            || ((posicion_contrincante.0 - self.fila).abs() == 2
                && (posicion_contrincante.1 - self.columna).abs() == 1)
    }
    fn dar_posicion(self) -> (i16, i16) {
        (self.fila, self.columna)
    }
}
