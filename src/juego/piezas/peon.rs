use super::{Color, AnalisisAtaque};

#[derive(Debug, Clone, Copy)]
pub struct Peon {
    fila: i16,
    columna: i16,
    color: Color,
}

impl Peon {
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
        //Valid Pawn move, if the piece moves from (X1, Y1) to (X2, Y2),

        if self.color == Color::Negro {
            (self.columna - posicion_contrincante.1).abs() == 1
                && posicion_contrincante.0 - self.fila == 1
        } else if self.color == Color::Blanco {
            (self.columna - posicion_contrincante.1).abs() == 1
                && self.fila - posicion_contrincante.0 == 1
        } else {
            println!("asodiaosd");
            std::process::exit(-1);
        }
    }
    fn dar_posicion(self) -> (i16, i16) {
        (self.fila, self.columna)
    }
}
