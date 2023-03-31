use crate::errors::TypeError;


//2 validacion
//que haya SOLO 2 caracteres, que uno sea minuscula y el otro mayuscula
pub fn estan_las_piezas_en(tablero: &[[char; 8]; 8]) -> Result<(), TypeError>{
    let (mut i , mut j)  = (0,0);
    let mut hay_dos_piezas = true;
    let mut hay_pieza_ajedrez = (false,false); //BLANCO NEGRO

    while i<8 && hay_dos_piezas{
        while j<8 && hay_dos_piezas{
            //print!("{} ",tablero[i][j]);
            if tablero[i][j].is_alphabetic() && !("RDACTPrdactp".contains(tablero[i][j])) {
                hay_dos_piezas = false;
            }else if "RDACTPrdactp".contains(tablero[i][j]){
                if tablero[i][j].is_lowercase() {hay_pieza_ajedrez.0 = true} else {hay_pieza_ajedrez.1 = true};
            }
            j += 1;
        }
        i += 1;
        j = 0;
    }
    //println!("cantidad de piezas {:?}",cantidad_de_piezas);
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