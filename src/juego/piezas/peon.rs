use super::{color::Color, AnalisisAtaque};

/// Contiene posicion y color de la pieza Peon
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Peon {
    fila: i16,
    columna: i16,
    color: Color,
}

impl Peon {
    ///Dada una posicion por parametro, crea la pieza Alfil
    pub fn new(f: i16, c: i16, color_ingresado: Color) -> Self {
        Peon {
            fila: f,
            columna: c,
            color: color_ingresado,
        }
    }
}

impl AnalisisAtaque for Peon {
    fn puedo_atacar_enemigo(&self, posicion_contrincante: (i16, i16)) -> bool {
        if self.color == Color::Negro {
            (self.columna - posicion_contrincante.1).abs() == 1
                && posicion_contrincante.0 - self.fila == 1
        } else {
            (self.columna - posicion_contrincante.1).abs() == 1
                && self.fila - posicion_contrincante.0 == 1
        }
    }
    fn dar_posicion(self) -> (i16, i16) {
        (self.fila, self.columna)
    }
}

#[cfg(test)]
mod tests {
    use crate::juego::piezas::{color::Color, peon::Peon, AnalisisAtaque};
    #[test]
    fn puedo_atacar_enemigo_peon_negro() {
        let pieza = Peon::new(1, 2, Color::Negro);
        assert_eq!(pieza.puedo_atacar_enemigo((2, 1)), true);
    }
    #[test]
    fn puedo_atacar_enemigo_peon_blanco() {
        let pieza = Peon::new(2, 2, Color::Blanco);
        assert_eq!(pieza.puedo_atacar_enemigo((1, 3)), true);
    }
    #[test]
    fn no_puedo_atacar_enemigo_peon() {
        let pieza = Peon::new(1, 1, Color::Negro);
        assert_eq!(pieza.puedo_atacar_enemigo((2, 1)), false);
    }
    #[test]
    fn no_puedo_atacar_enemigo_peon_negro() {
        let pieza = Peon::new(1, 1, Color::Blanco);
        assert_eq!(pieza.puedo_atacar_enemigo((2, 2)), false);
    }
    #[test]
    fn no_puedo_atacar_enemigo_peon_blanco() {
        let pieza = Peon::new(2, 2, Color::Negro);
        assert_eq!(pieza.puedo_atacar_enemigo((1, 1)), false);
    }
}
