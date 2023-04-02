use super::AnalisisAtaque;

/// Contiene posicion de la pieza Caballo
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Caballo {
    fila: i16,
    columna: i16,
}

impl Caballo {
    ///Dada una posicion por parametro, crea la pieza Alfil
    pub fn new(f: i16, c: i16) -> Self {
        Caballo {
            fila: f,
            columna: c,
        }
    }
}

impl AnalisisAtaque for Caballo {
    fn puedo_atacar_enemigo(&self, posicion_contrincante: (i16, i16)) -> bool {
        ((posicion_contrincante.0 - self.fila).abs() == 1
            && (posicion_contrincante.1 - self.columna).abs() == 2)
            || ((posicion_contrincante.0 - self.fila).abs() == 2
                && (posicion_contrincante.1 - self.columna).abs() == 1)
    }
    fn dar_posicion(self) -> (i16, i16) {
        (self.fila, self.columna)
    }
}

#[cfg(test)]
mod tests {
    use crate::juego::piezas::{caballo::Caballo, AnalisisAtaque};
    #[test]
    fn puedo_atacar_enemigo_caballo() {
        let mut pieza = Caballo::new(1, 1);
        assert_eq!(pieza.puedo_atacar_enemigo((3, 2)), true);
        pieza = Caballo::new(3, 1);
        assert_eq!(pieza.puedo_atacar_enemigo((2, 3)), true);
    }
    #[test]
    fn no_puedo_atacar_enemigo_caballo() {
        let pieza = Caballo::new(2, 2);
        assert_eq!(pieza.puedo_atacar_enemigo((1, 2)), false);
    }
}
