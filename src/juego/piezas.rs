pub(crate) mod rey;



#[derive(Debug)]
pub enum Color{
    Blanco,
    Negro,
}


#[derive(Debug)]
pub struct Dama{
    fila: i32,
    columna: i32,
    color: Color,
}
#[derive(Debug)]
pub struct Alfil{
    fila: i32,
    columna: i32,
    color: Color,
}
#[derive(Debug)]
pub struct Caballo{
    fila: i32,
    columna: i32,
    color: Color,
}
#[derive(Debug)]
pub struct Torre{
    fila: i32,
    columna: i32,
    color: Color,
}
#[derive(Debug)]
pub struct Peon{
    fila: i32,
    columna: i32,
    color: Color,
} //TODO: Ver que solo peon necesita color

#[derive(Debug)]
pub enum PiezaAjedrez{
    Rey(rey::Rey),
    Dama(Dama),
    Alfil(Alfil),
    Caballo(Caballo),
    Torre(Torre),
    Peon(Peon),
}
impl PiezaAjedrez{
    pub fn puede_atacar(&self, pieza_contrincante: PiezaAjedrez){
        match &self {
            Self::Rey (pieza_rey) => {}
            Self::Dama (pieza_reina) => {}
            Self::Alfil (pieza_alfil) => {}
            Self::Caballo (pieza_caballo) => {}
            Self::Torre (pieza_torre) => {}
            Self::Peon (pieza_peon) => {}
            _ => {}
        }
    }
}



pub trait AnalisisAtaque{
    fn ver_si_atacar_enemigo(&self, pieza_contrincante: PiezaAjedrez) -> bool;
}

impl AnalisisAtaque for Dama{
    fn ver_si_atacar_enemigo(&self, pieza_contrincante: PiezaAjedrez) -> bool{
        true
    }
}






























pub fn funcion_prueba(){

    //let pieza_negra = PiezaAjedrez::Rey(rey::Rey{fila: 2, columna: 2,color: Color::Negro});
    //let pieza_blanca = PiezaAjedrez::Caballo(Caballo{fila: 5, columna: 2,color: Color::Blanco});

    //println!("salio de funcion_prueba {:?}{:?}",pieza_blanca,pieza_negra);
}

//pub struct Posicion(pub i32,pub i32);