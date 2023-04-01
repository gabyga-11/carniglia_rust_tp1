pub mod piezas;
use self::piezas::{
    alfil::Alfil, caballo::Caballo, dama::Dama, peon::Peon, rey::Rey, torre::Torre, Color,
    PiezaAjedrez,
};
use crate::errors::TypeError;

const CHARS_AJEDREZ: &str = "RDACTPrdactp";

pub struct Juego {
    pieza_blanca: piezas::PiezaAjedrez,
    pieza_negra: piezas::PiezaAjedrez,
    tablero: [[char; 8]; 8],
}

impl Juego {
    pub fn new(table: [[char; 8]; 8]) -> Self {
        Juego {
            tablero: table,
            pieza_blanca: piezas::PiezaAjedrez::Indefinida,
            pieza_negra: piezas::PiezaAjedrez::Indefinida,
        }
    }

    pub fn definir_piezas_en_tablero(&mut self) -> Result<(), TypeError> {
        let (mut i, mut j) = (0, 0);
        let mut hay_dos_piezas = true;
        let mut hay_pieza_ajedrez = (false, false); //BLANCO NEGRO
        let tablero = self.tablero;

        while i < 8 && hay_dos_piezas {
            while j < 8 && hay_dos_piezas {
                if tablero[i][j].is_alphabetic() && !(CHARS_AJEDREZ.contains(tablero[i][j])) {
                    hay_dos_piezas = false;
                } else if CHARS_AJEDREZ.contains(tablero[i][j]) {
                    if tablero[i][j].is_lowercase() {
                        hay_pieza_ajedrez.0 = true
                    } else {
                        hay_pieza_ajedrez.1 = true
                    };
                    self.cargar_pieza(
                        &tablero[i][j],
                        i16::try_from(i).unwrap_or_default(),
                        i16::try_from(j).unwrap_or_default(),
                    );
                }
                j += 1;
            }
            i += 1;
            j = 0;
        }
        analizar_chequeo_tablero(hay_dos_piezas, hay_pieza_ajedrez)
    }

    pub fn cargar_pieza(&mut self, char_pieza: &char, fila: i16, col: i16) {
        let pieza_en_tablero = match char_pieza {
            'r' | 'R' => PiezaAjedrez::Rey(Rey::new(fila, col)),
            'd' | 'D' => PiezaAjedrez::Dama(Dama::new(fila, col)),
            'a' | 'A' => PiezaAjedrez::Alfil(Alfil::new(fila, col)),
            'c' | 'C' => PiezaAjedrez::Caballo(Caballo::new(fila, col)),
            't' | 'T' => PiezaAjedrez::Torre(Torre::new(fila, col)),
            'p' => PiezaAjedrez::Peon(Peon::new(fila, col, Color::Blanco)),
            'P' => PiezaAjedrez::Peon(Peon::new(fila, col, Color::Negro)),
            _ => PiezaAjedrez::Indefinida,
        };
        if char_pieza.is_lowercase() {
            self.pieza_blanca = pieza_en_tablero;
        } else {
            self.pieza_negra = pieza_en_tablero;
        }
    }

    pub fn analisis_de_ataques(&self) -> (bool, bool) {
        let blanca_puede_atacar = self.pieza_blanca.puede_atacar(&(self.pieza_negra));
        let negra_puede_atacar = self.pieza_negra.puede_atacar(&(self.pieza_blanca));
        (blanca_puede_atacar, negra_puede_atacar)
    }

    pub fn reportar_resultado(&self, puede_capturar: (bool, bool)) {
        match puede_capturar {
            (true, true) => {
                println!("E");
            }
            (false, false) => {
                println!("P");
            }
            (true, false) => {
                println!("B");
            }
            (false, true) => {
                println!("N");
            }
        }
    }
}

fn analizar_chequeo_tablero(
    hay_dos_piezas: bool,
    hay_pieza_ajedrez: (bool, bool),
) -> Result<(), TypeError> {
    if !hay_dos_piezas {
        Err(TypeError::PiezaInexistenteEnAjedrez)
    } else if hay_pieza_ajedrez.0 && !hay_pieza_ajedrez.1 {
        Err(TypeError::HayDosPiezasBlancas)
    } else if !hay_pieza_ajedrez.0 && hay_pieza_ajedrez.1 {
        Err(TypeError::HayDosPiezasNegras)
    } else {
        Ok(())
    }
}
