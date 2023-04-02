pub mod alfil;
pub mod caballo;
pub mod color;
pub mod dama;
pub mod peon;
pub mod rey;
pub mod torre;
use self::{alfil::Alfil, caballo::Caballo, dama::Dama, peon::Peon, rey::Rey, torre::Torre};

/// PiezaAjedrez es un enum donde cada valor se corresponde con una pieza del Ajedrez,
/// mas una pieza indefinida. 
/// 
/// Dentro de cada pieza, se encuentra un struct del tipo de pieza que corresponda.
/// 
/// ### Ejemplo
/// ```
/// let pieza_blanca = PiezaAjedrez::Caballo(Caballo::new(4, 4));
/// assert_eq!(pieza_blanca.posicion(),(4,4));
/// ```
/// 
#[derive(Debug, PartialEq)]
pub enum PiezaAjedrez {
    Rey(Rey),
    Dama(Dama),
    Alfil(Alfil),
    Caballo(Caballo),
    Torre(Torre),
    Peon(Peon),
    Indefinida,
}
impl PiezaAjedrez {
    /// Dada una pieza contrincante, lo que hace es preguntarle a la
    /// pieza contenida (struct dentro del enum PiezaAjedrez)
    /// si puede atacar a la pieza contrincante.
    /// 
    /// Retorna true o false en base al resultado del analisis de ataque de
    /// la pieza contenida
    pub fn puede_atacar(&self, pieza_contrincante: &PiezaAjedrez) -> bool {
        match &self {
            Self::Rey(pieza_rey) => pieza_rey.puedo_atacar_enemigo(pieza_contrincante.posicion()),
            Self::Dama(pieza_reina) => {
                pieza_reina.puedo_atacar_enemigo(pieza_contrincante.posicion())
            }
            Self::Alfil(pieza_alfil) => {
                pieza_alfil.puedo_atacar_enemigo(pieza_contrincante.posicion())
            }
            Self::Caballo(pieza_caballo) => {
                pieza_caballo.puedo_atacar_enemigo(pieza_contrincante.posicion())
            }
            Self::Torre(pieza_torre) => {
                pieza_torre.puedo_atacar_enemigo(pieza_contrincante.posicion())
            }
            Self::Peon(pieza_peon) => {
                pieza_peon.puedo_atacar_enemigo(pieza_contrincante.posicion())
            }
            Self::Indefinida => false,
        }
    }
    /// Accede al struct de la pieza que corresponda
    /// y le pide la posicion en el tablero, donde (0,0)
    /// es arriba a la izquierda.
    pub fn posicion(&self) -> (i16, i16) {
        match self {
            Self::Rey(pieza_rey) => pieza_rey.dar_posicion(),
            Self::Dama(pieza_reina) => pieza_reina.dar_posicion(),
            Self::Alfil(pieza_alfil) => pieza_alfil.dar_posicion(),
            Self::Caballo(pieza_caballo) => pieza_caballo.dar_posicion(),
            Self::Torre(pieza_torre) => pieza_torre.dar_posicion(),
            Self::Peon(pieza_peon) => pieza_peon.dar_posicion(),
            Self::Indefinida => (0, 0),
        }
    }
}

/// Grupo de funciones asociadas a todas las piezas
/// de ajedrez en su conjunto. 
/// 
/// Relativo a lo necesario para definir
/// si el ataque de una pieza a otra puede realizarse.
/// 
/// #### - puedo_atacar_enemigo
/// Esta funcion determina, dada la posicion de la pieza contrincante,
/// si la pieza en cuestion puede atacarla o no. Retorna resultado en un booleano
/// #### - dar_posicion.
/// Retorna la posición en el tablero de la pieza.
pub trait AnalisisAtaque {
    fn puedo_atacar_enemigo(&self, posicion_contrincante: (i16, i16)) -> bool;
    fn dar_posicion(self) -> (i16, i16);
}
