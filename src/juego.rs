pub mod piezas;
use self::piezas::{
    alfil::Alfil, caballo::Caballo, color::Color, dama::Dama, peon::Peon, rey::Rey, torre::Torre,
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
        self.analizar_chequeo_tablero(hay_dos_piezas, hay_pieza_ajedrez)
    }

    fn cargar_pieza(&mut self, char_pieza: &char, fila: i16, col: i16) {
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

    fn analizar_chequeo_tablero(
        &self,
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
}

#[test]
pub fn definir_piezas_en_tablero_ok() {
    let tablero = [
        ['c', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', 'P', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
    ];
    let mut juego = Juego::new(tablero);

    assert_eq!(juego.definir_piezas_en_tablero(), Ok(()));
}

#[test]
pub fn definir_piezas_en_tablero_peon() {
    let tablero = [
        ['p', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', 'P', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
    ];
    let mut juego = Juego::new(tablero);
    juego.definir_piezas_en_tablero().unwrap();
    assert_eq!(juego.pieza_negra, PiezaAjedrez::Peon(Peon::new(4,4,Color::Negro)));
    assert_eq!(juego.pieza_blanca, PiezaAjedrez::Peon(Peon::new(0,0,Color::Blanco)));
}

#[test]
pub fn definir_piezas_en_tablero_caballo() {
    let tablero = [
        ['c', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', 'C', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
    ];
    let mut juego = Juego::new(tablero);
    juego.definir_piezas_en_tablero().unwrap();
    assert_eq!(juego.pieza_negra, PiezaAjedrez::Caballo(Caballo::new(4,4)));
    assert_eq!(juego.pieza_blanca, PiezaAjedrez::Caballo(Caballo::new(0,0)));
}

#[test]
pub fn definir_piezas_en_tablero_alfil() {
    let tablero = [
        ['_', 'a', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', 'A', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
    ];
    let mut juego = Juego::new(tablero);
    juego.definir_piezas_en_tablero().unwrap();
    assert_eq!(juego.pieza_negra, PiezaAjedrez::Alfil(Alfil::new(4,4)));
    assert_eq!(juego.pieza_blanca, PiezaAjedrez::Alfil(Alfil::new(0,1)));
}

#[test]
pub fn definir_piezas_en_tablero_rey() {
    let tablero = [
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['R', '_', '_', '_', '_', '_', '_', 'r'],
    ];
    let mut juego = Juego::new(tablero);
    juego.definir_piezas_en_tablero().unwrap();
    assert_eq!(juego.pieza_negra, PiezaAjedrez::Rey(Rey::new(7,0)));
    assert_eq!(juego.pieza_blanca, PiezaAjedrez::Rey(Rey::new(7,7)));
}

#[test]
pub fn definir_piezas_en_tablero_dama() {
    let tablero = [
        ['d', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', 'D', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
    ];
    let mut juego = Juego::new(tablero);
    juego.definir_piezas_en_tablero().unwrap();
    assert_eq!(juego.pieza_negra, PiezaAjedrez::Dama(Dama::new(5,5)));
    assert_eq!(juego.pieza_blanca, PiezaAjedrez::Dama(Dama::new(0,0)));
}

#[test]
pub fn definir_piezas_en_tablero_torre() {
    let tablero = [
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', 't', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['T', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
    ];
    let mut juego = Juego::new(tablero);
    juego.definir_piezas_en_tablero().unwrap();
    assert_eq!(juego.pieza_negra, PiezaAjedrez::Torre(Torre::new(3,0)));
    assert_eq!(juego.pieza_blanca, PiezaAjedrez::Torre(Torre::new(1,6)));
}



#[test]
pub fn definir_piezas_en_tablero_inexistente() {
    let tablero = [
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', 'P', 'q', '_', '_'],
        ['_', '_', '_', '_', '_', 'p', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
    ];
    let mut juego = Juego::new(tablero);

    assert_eq!(
        juego.definir_piezas_en_tablero(),
        Err(TypeError::PiezaInexistenteEnAjedrez)
    );
}
#[test]
pub fn definir_piezas_en_tablero_dos_negras() {
    let tablero = [
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', 'A', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', 'P', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
    ];
    let mut juego = Juego::new(tablero);

    assert_eq!(
        juego.definir_piezas_en_tablero(),
        Err(TypeError::HayDosPiezasNegras)
    );
}

#[test]
pub fn definir_piezas_en_tablero_dos_blancas() {
    let tablero = [
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', 'c', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
        ['p', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_'],
    ];
    let mut juego = Juego::new(tablero);

    assert_eq!(
        juego.definir_piezas_en_tablero(),
        Err(TypeError::HayDosPiezasBlancas)
    );
}
