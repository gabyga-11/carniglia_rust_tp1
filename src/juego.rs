use std::env::consts::FAMILY;

use crate::{errors::TypeError, juego::piezas::{dama::Dama, alfil::Alfil, caballo::Caballo, torre::Torre, peon::Peon, Color}};


pub mod piezas;
use self::piezas::PiezaAjedrez;
use piezas::rey::Rey;

//2 validacion
//que haya SOLO 2 caracteres, que uno sea minuscula y el otro mayuscula


pub struct Juego{
    pieza_blanca:  piezas::PiezaAjedrez,
    pieza_negra: piezas::PiezaAjedrez,
    tablero: [[char; 8]; 8],
}

impl Juego{

    pub fn new(table: [[char; 8]; 8]) -> Self{
        Juego{tablero: table, pieza_blanca: piezas::PiezaAjedrez::Indefinida, pieza_negra: piezas::PiezaAjedrez::Indefinida}
    }


    pub fn definir_piezas_en_tablero(&mut self) -> Result<(), TypeError>{

        let (mut i , mut j  )  = (0,0);
        let mut hay_dos_piezas = true;
        let mut hay_pieza_ajedrez = (false,false); //BLANCO NEGRO
        let tablero = self.tablero;


        while i<8 && hay_dos_piezas{
            while j<8 && hay_dos_piezas{
                //print!("{} ",tablero[i][j]);
                if tablero[i][j].is_alphabetic() && !("RDACTPrdactp".contains(tablero[i][j])) {
                    hay_dos_piezas = false;
                }else if "RDACTPrdactp".contains(tablero[i][j]){
                    if tablero[i][j].is_lowercase() { hay_pieza_ajedrez.0 = true } else { hay_pieza_ajedrez.1 = true };
                    self.cargar_pieza(&tablero[i][j],i,j);
                }
                j += 1;
            }
            i += 1;
            j = 0;
        }
        analizar_chequeo_tablero(hay_dos_piezas, hay_pieza_ajedrez)

    }


    pub fn cargar_pieza(&mut self, char_pieza: &char, fila: usize, col: usize){
        
        let pieza_en_tablero = match char_pieza {
            'r' | 'R' => {
                let rey = Rey::new(fila,col);
                PiezaAjedrez::Rey(rey)
            },
            'd' | 'D' => {
                let dama = Dama::new(fila,col);
                PiezaAjedrez::Dama(dama)
            },
            'a' | 'A' => {
                let alfil = Alfil::new(fila,col);
                PiezaAjedrez::Alfil(alfil)
            },
            'c' | 'C' => {
                let caballo = Caballo::new(fila,col);
                PiezaAjedrez::Caballo(caballo)
            },
            't' | 'T' => {
                let torre = Torre::new(fila,col);
                PiezaAjedrez::Torre(torre)
            },
            'p' => {
                let peon = Peon::new(fila,col,Color::Blanco);
                PiezaAjedrez::Peon(peon)
            },
            'P' => {
                let peon = Peon::new(fila,col,Color::Negro);
                PiezaAjedrez::Peon(peon)
            },
            _ => PiezaAjedrez::Indefinida,
        };
        if char_pieza.is_lowercase(){
            self.pieza_blanca = pieza_en_tablero;
        }else{
            self.pieza_negra = pieza_en_tablero;
        }
        println!("{:?}",self.pieza_blanca);
        println!("{:?}",self.pieza_negra);
    }


    pub fn analisis_de_ataques(&self) -> (bool, bool){
        let blanca_puede_atacar = self.pieza_blanca.puede_atacar(&(self.pieza_negra));
        let negra_puede_atacar= self.pieza_negra.puede_atacar(&(self.pieza_blanca));
        (blanca_puede_atacar, negra_puede_atacar)
    }

    pub fn reportar_resultado(&self, puede_capturar: (bool, bool)){
        match puede_capturar{
            (true,true) => { println!("E")},
            (false,false) => { println!("P")},
            (true,false) => { println!("B")},
            (false,true) => { println!("N")},
        }
    }

}

fn analizar_chequeo_tablero(hay_dos_piezas: bool, hay_pieza_ajedrez: (bool, bool)) -> Result<(), TypeError>{
    if !hay_dos_piezas{
        Err(TypeError::PiezaInexistenteEnAjedrez)  
    }else if hay_pieza_ajedrez.0 && !hay_pieza_ajedrez.1 {
        Err(TypeError::HayDosPiezasBlancas)
    }else if !hay_pieza_ajedrez.0 && hay_pieza_ajedrez.1{
        Err(TypeError::HayDosPiezasNegras)
    }else{
        Ok(())
    }
}