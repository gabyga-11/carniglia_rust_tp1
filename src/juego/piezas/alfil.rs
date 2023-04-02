use super::AnalisisAtaque;

/// Contiene posicion de la pieza Alfil
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Alfil {
    fila: i16,
    columna: i16,
}

impl Alfil {
    ///Dada una posicion por parametro, crea la pieza Alfil
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



#[cfg(test)]
mod tests {
    use crate::juego::piezas::{alfil::Alfil, AnalisisAtaque};
    #[test]
    fn puedo_atacar_enemigo_alfil() {
        let mut pieza = Alfil::new(1,1);
        assert_eq!(pieza.puedo_atacar_enemigo((5,5)), true);
        pieza = Alfil::new(7,7);
        assert_eq!(pieza.puedo_atacar_enemigo((5,5)), true);
    }
    #[test]
    fn no_puedo_atacar_enemigo_alfil() {
        let pieza = Alfil::new(1,1);
        assert_eq!(pieza.puedo_atacar_enemigo((1,2)), false);
    }
}