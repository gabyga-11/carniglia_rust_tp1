use super::AnalisisAtaque;

/// Contiene posicion de la pieza Torre
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Torre {
    fila: i16,
    columna: i16,
}

impl Torre {
    ///Dada una posicion por parametro, crea la pieza Alfil
    pub fn new(f: i16, c: i16) -> Self {
        Torre {
            fila: f,
            columna: c,
        }
    }
}

impl AnalisisAtaque for Torre {
    fn puedo_atacar_enemigo(&self, posicion_contrincante: (i16, i16)) -> bool {
        self.fila == posicion_contrincante.0 || self.columna == posicion_contrincante.1
    }
    fn dar_posicion(self) -> (i16, i16) {
        (self.fila, self.columna)
    }
}

#[cfg(test)]
mod tests {
    use crate::juego::piezas::{torre::Torre, AnalisisAtaque};
    #[test]
    fn puedo_atacar_enemigo_torre() {
        let mut pieza = Torre::new(1, 1);
        assert_eq!(pieza.puedo_atacar_enemigo((3, 1)), true);
        pieza = Torre::new(1, 1);
        assert_eq!(pieza.puedo_atacar_enemigo((1, 7)), true);
    }
    #[test]
    fn no_puedo_atacar_enemigo_torre() {
        let pieza = Torre::new(1, 1);
        assert_eq!(pieza.puedo_atacar_enemigo((3, 2)), false);
    }
}
