pub mod rey;
pub mod dama;
pub mod caballo;
pub mod alfil;
pub mod peon;
pub mod torre;
use self::{rey::Rey, dama::Dama, alfil::Alfil, caballo::Caballo, peon::Peon, torre::Torre};

#[derive(Debug,Clone,Copy)]
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
    pub fn puede_atacar(&self, pieza_contrincante: &PiezaAjedrez) -> bool{
        
        match &self {
            Self::Rey (pieza_rey) => {pieza_rey.puedo_atacar_enemigo(pieza_contrincante.posicion())}
            Self::Dama (pieza_reina) => {pieza_reina.puedo_atacar_enemigo(pieza_contrincante.posicion())}
            Self::Alfil (pieza_alfil) => {pieza_alfil.puedo_atacar_enemigo(pieza_contrincante.posicion())}
            Self::Caballo (pieza_caballo) => {pieza_caballo.puedo_atacar_enemigo(pieza_contrincante.posicion())}
            Self::Torre (pieza_torre) => {pieza_torre.puedo_atacar_enemigo(pieza_contrincante.posicion())}
            Self::Peon (pieza_peon) => {pieza_peon.puedo_atacar_enemigo(pieza_contrincante.posicion())}
            _ => {false}
        }
    }


    pub fn posicion(&self) -> (usize,usize) {
        match self {
            Self::Rey (pieza_rey) => { pieza_rey.dar_posicion()}
            Self::Dama (pieza_reina) => {pieza_reina.dar_posicion()}
            Self::Alfil (pieza_alfil) => {pieza_alfil.dar_posicion()}
            Self::Caballo (pieza_caballo) => {pieza_caballo.dar_posicion()}
            Self::Torre (pieza_torre) => {pieza_torre.dar_posicion()}
            Self::Peon (pieza_peon) => {pieza_peon.dar_posicion()}
            _ => {(0,0)}
        }
    }
}

pub trait AnalisisAtaque{
    fn puedo_atacar_enemigo(&self, posicion_contrincante: (usize, usize)) -> bool;
    fn dar_posicion(self) -> (usize, usize);
}
