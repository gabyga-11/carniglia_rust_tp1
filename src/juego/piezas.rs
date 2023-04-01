pub mod rey;
pub mod dama;
pub mod caballo;
pub mod alfil;
pub mod peon;
pub mod torre;
use self::{rey::Rey, dama::Dama, alfil::Alfil, caballo::Caballo, peon::Peon, torre::Torre};


#[derive(Debug)]
pub enum Color{
    Blanco,
    Negro,
}

#[derive(Debug)]
pub enum PiezaAjedrez{
    Rey(Rey),
    Dama(Dama),
    Alfil(Alfil),
    Caballo(Caballo),
    Torre(Torre),
    Peon(Peon),
    Indefinida,
}
impl PiezaAjedrez{
    pub fn puede_atacar(&self, pieza_contrincante: PiezaAjedrez){

        match &self {
            Self::Rey (pieza_rey) => {pieza_rey.puedo_atacar_enemigo(pieza_contrincante);}
            Self::Dama (pieza_reina) => {}
            Self::Alfil (pieza_alfil) => {}
            Self::Caballo (pieza_caballo) => {}
            Self::Torre (pieza_torre) => {}
            Self::Peon (pieza_peon) => {}
            _ => {}
        }
    }
    pub fn estas_en_casilla(f: usize, c: usize){
        
    }
}

pub trait AnalisisAtaque{
    fn puedo_atacar_enemigo(&self, pieza_contrincante: PiezaAjedrez) -> bool;
}
