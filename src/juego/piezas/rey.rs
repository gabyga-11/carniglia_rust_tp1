use super::AnalisisAtaque;

/// Contiene posicion de la pieza Rey
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rey {
    fila: i16,
    columna: i16,
}

impl Rey {
    ///Dada una posicion por parametro, crea la pieza Alfil
    pub fn new(f: i16, c: i16) -> Self {
        Rey {
            fila: f,
            columna: c,
        }
    }
}

impl AnalisisAtaque for Rey {
    fn puedo_atacar_enemigo(&self, posicion_contrincante: (i16, i16)) -> bool {
        (self.fila - posicion_contrincante.0).abs() <= 1
            && (self.columna - posicion_contrincante.1).abs() <= 1
    }
    fn dar_posicion(self) -> (i16, i16) {
        (self.fila, self.columna)
    }
}

#[cfg(test)]
mod tests {
    use crate::juego::piezas::{rey::Rey, AnalisisAtaque};
    #[test]
    fn puedo_atacar_enemigo_rey() {
        let mut pieza = Rey::new(4, 4);
        assert_eq!(pieza.puedo_atacar_enemigo((3, 3)), true);
        pieza = Rey::new(6, 6);
        assert_eq!(pieza.puedo_atacar_enemigo((7, 6)), true);
    }
    #[test]
    fn no_puedo_atacar_enemigo_rey() {
        let pieza = Rey::new(1, 1);
        assert_eq!(pieza.puedo_atacar_enemigo((3, 2)), false);
    }
}
