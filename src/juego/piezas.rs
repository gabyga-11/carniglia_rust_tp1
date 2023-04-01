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
    Innecesario,
}
/* 
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
    /*pub fn get_pieza(){
        if let PiezaAjedrez::Circle(elemento){
            
        }
    }*/
}

*/

pub trait AnalisisAtaque{
    fn puedo_atacar_enemigo(&self, pieza_contrincante: NuevaPiezaAjedrez<()>) -> bool;
}


//###########################################
/* 
enum nuevoTipoPieza{
    Rey,
    Dama,
    Alfil,
    Caballo,
    Torre,
    Peon,
}*/




#[derive(Debug)]
pub struct NuevaPiezaAjedrez<T>{
    tipo: T,

}
impl NuevaPiezaAjedrez<Rey>{
    pub fn new(f: usize, c: usize) -> Self{
        NuevaPiezaAjedrez{tipo: Rey::new( f, c)}
    }
}
impl NuevaPiezaAjedrez<Dama>{
    pub fn new(f: usize, c: usize) -> Self{
        NuevaPiezaAjedrez{tipo: Dama::new( f, c)}
    }
}
impl NuevaPiezaAjedrez<Torre>{
    pub fn new(f: usize, c: usize) -> Self{
        NuevaPiezaAjedrez{tipo: Torre::new( f, c)}
    }
}
impl NuevaPiezaAjedrez<Caballo>{
    pub fn new(f: usize, c: usize) -> Self{
        NuevaPiezaAjedrez{tipo: Caballo::new( f, c)}
    }
}
impl NuevaPiezaAjedrez<Alfil>{
    pub fn new(f: usize, c: usize) -> Self{
        NuevaPiezaAjedrez{tipo: Alfil::new( f, c)}
    }
}
impl NuevaPiezaAjedrez<Peon>{
    pub fn new(f: usize, c: usize, color: Color) -> Self{
        NuevaPiezaAjedrez{tipo: Peon::new( f, c, color)}
    }
}
/* 
pub struct PiezaAjedrezBuilder{
    tipo: 
}impl PiezaAjedrezBuilder{
    pub fn new() -> Self {
        PiezaAjedrezBuilder {
            recipiente: Recipiente::Vaso,
            gustos: vec![],
            cucharita: false,
        }
    }
}*/