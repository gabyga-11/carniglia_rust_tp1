use super::AnalisisAtaque;

#[derive(Debug, Clone, Copy)]
pub struct Alfil {
    fila: i16,
    columna: i16,
}

impl Alfil {
    pub fn new(f: i16, c: i16) -> Self {
        Alfil {
            fila: f,
            columna: c,
        }
    }
}

impl AnalisisAtaque for Alfil {
    fn puedo_atacar_enemigo(&self, posicion_contrincante: (i16, i16)) -> bool {
        (self.fila - posicion_contrincante.0).abs()
            == (self.columna - posicion_contrincante.1).abs()
    }
    fn dar_posicion(self) -> (i16, i16) {
        (self.fila, self.columna)
    }
}
