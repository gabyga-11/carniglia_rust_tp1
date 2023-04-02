use super::AnalisisAtaque;

/// Contiene posicion de la pieza Dama
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dama {
    fila: i16,
    columna: i16,
}

impl Dama {
    ///Dada una posicion por parametro, crea la pieza Alfil
    pub fn new(f: i16, c: i16) -> Self {
        Dama {
            fila: f,
            columna: c,
        }
    }
}

impl AnalisisAtaque for Dama {
    fn puedo_atacar_enemigo(&self, posicion_contrincante: (i16, i16)) -> bool {
        (self.fila == posicion_contrincante.0 || self.columna == posicion_contrincante.1)
            || ((self.fila - posicion_contrincante.0).abs()
                == (self.columna - posicion_contrincante.1).abs())
    }

    fn dar_posicion(self) -> (i16, i16) {
        (self.fila, self.columna)
    }
}


#[cfg(test)]
mod tests {
    use crate::juego::piezas::{dama::Dama, AnalisisAtaque};
    #[test]
    fn puedo_atacar_enemigo_dama() {
        let mut pieza = Dama::new(1,1);
        assert_eq!(pieza.puedo_atacar_enemigo((3,1)), true);
        pieza = Dama::new(5,5);
        assert_eq!(pieza.puedo_atacar_enemigo((7,7)), true);
    }
    #[test]
    fn no_puedo_atacar_enemigo_dama() {
        let pieza = Dama::new(1,1);
        assert_eq!(pieza.puedo_atacar_enemigo((3,2)), false);
    }
}