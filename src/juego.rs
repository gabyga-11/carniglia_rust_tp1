use crate::{errors::TypeError, juego::piezas::{dama::Dama, alfil::Alfil, caballo::Caballo, torre::Torre, peon::Peon, Color}};


pub mod piezas;
use self::piezas::NuevaPiezaAjedrez;
use piezas::rey::Rey;


pub struct Juego{ //TODO: saca pub cuando hayas arreglado lo del builder
    //pub pieza_blanca:  piezas::NuevaPiezaAjedrez<Rey>,
    //pub pieza_negra: piezas::NuevaPiezaAjedrez<Rey>, 
    //TODO: COn el refacto va a romper todo esto!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    //Es necesario hacer un builder para el juego, desgraciadamente
    pub tablero: [[char; 8]; 8],
}

impl Juego{

    //pub fn new(table: [[char; 8]; 8]) -> Self{
    //    
    //    Juego{tablero: table, pieza_blanca: piezas::NuevaPiezaAjedrez<Rey>::new , pieza_negra: piezas::NuevaPiezaAjedrez<>} 
    //}
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
                    self.cargar_pieza(tablero[i][j],i,j)?;
                }
                j += 1;
            }
            i += 1;
            j = 0;
        }
        analizar_chequeo_tablero(hay_dos_piezas, hay_pieza_ajedrez)
    
    }

    


    pub fn cargar_pieza(&mut self, char_pieza: char, fila: usize, col: usize) -> Result<(), TypeError>{
        //let pieza_en_tablero;
        if char_pieza == 'r' || char_pieza == 'R'{
            Ok(())
        }else{
            Err(TypeError::PiezaInexistenteEnAjedrezCargado)
        }
        /* 
        match char_pieza {
            'r' | 'R' => {
                let rey = Rey::new(fila,col);
                
                pieza_en_tablero = NuevaPiezaAjedrez{tipo: rey};
            },
            'd' | 'D' => {
                let dama = Dama::new(fila,col);
                pieza_en_tablero = NuevaPiezaAjedrez{tipo: dama};
            },
            'a' | 'A' => {
                let alfil = Alfil::new(fila,col);
                pieza_en_tablero = NuevaPiezaAjedrez{tipo: alfil};
            },
            'c' | 'C' => {
                let caballo = Caballo::new(fila,col);
                pieza_en_tablero = NuevaPiezaAjedrez{tipo: caballo};
            },
            't' | 'T' => {
                let torre = Torre::new(fila,col);
                pieza_en_tablero = NuevaPiezaAjedrez{tipo: torre};
            },
            'p' => {
                let peon = Peon::new(fila,col,Color::Blanco);
                let pieza_en_tablero = NuevaPiezaAjedrez{tipo: peon};
            },
            'P' => {
                let peon = Peon::new(fila,col,Color::Negro);
                let pieza_en_tablero = NuevaPiezaAjedrez{tipo: peon};
            },
            //_ => PiezaAjedrez::Indefinida,
        }*/


        //TODO: para poder continuar, necesito que el builder cargue los datos ACA
        //if char_pieza.is_lowercase(){
            //self.pieza_blanca = pieza_en_tablero;
        //}else{
            //self.pieza_negra = pieza_en_tablero;
        //}
        //println!("{:?}",self.pieza_blanca);
        //println!("{:?}",self.pieza_negra);
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

