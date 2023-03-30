use std::{fs::{self}, io::{ErrorKind}, process::exit};
use crate::errors::TypeError;

pub struct FileHandler {
    path: String,
}

impl FileHandler{

    pub fn new(x: Vec<String>) -> Self{
        if x.len() != 2 {
            println!("ERROR: Ingrese un único nombre de archivo");
            exit(0);
        }
        let pathfile = format!("{}{}","src/".to_owned(),x[1]);
        FileHandler{path: pathfile}
    }


    pub fn leer(&self) -> Result<(), TypeError > {

    
        //let path = self.generar_path(self.path.as_str())?;
    
        let contenido = self.leer_archivo_completo(self.path.as_str())?;



        //for linea in contenido.lines(){
          //  println!("El contenido es {}",linea);
        //}



        let string_prueba = String::from("_ _ _ _ _ _ _ _"); //TODO: VER ACA
        println!("{:?}",string_prueba.matches("_").count());
        //VALIDACIONES
        //1 validacion:
        //debe haber siempre 7 " "   y ademas 6,7,8 "_"   
        //ademas,iterar y verificar que los espacios esten en las posiciones impares de iteracion

        //OJO QUE DEBE ESTAR SIEMPRE INTERCALADOS LOS "_" CON " "

        //2 validacion
        //DEBE HAber un total de 62 guiones -> hay mas de 2 piezas en el tablero
        //que haya SOLO 2 caracteres, que uno sea minuscula y el otro mayuscula

        let mut tablero = [[' ' as char; 8]; 8];
        for (indice_linea,linea) in contenido.lines().enumerate(){
            
            for (indice_letra,letra) in linea.split(" ").enumerate(){
                
                tablero[indice_linea][indice_letra] = letra.chars().next().unwrap_or_default();
                print!("{} ",tablero[indice_linea][indice_letra]);
            }
            println!("\n");
        }

        Ok(())
    }
    
    /* 
    fn generar_path(&self, filepath: &str) -> Result<String, TypeError > {
        let full_path = format!("{}{}",String::from("src/"),filepath);
        
        if filepath == "c" { //si no termina en .txt
            Err(TypeError::ErrorPath)
        }else{
            Ok(full_path)
        }
    
    }*/
    
    fn leer_archivo_completo(&self, filepath: &str) -> Result<String, TypeError> {
        println!("{:?}",filepath);
        let archivo_resultado = fs::read_to_string(filepath);
        let resultado_final = match archivo_resultado {
            Ok(archivo) => {
                Ok(archivo)
            }
            Err(error_obtenido) => {
                if error_obtenido.kind() == ErrorKind::NotFound{
                    Err(TypeError::ErrorNombreDeArchivoInvalido)
                }else{
                    Err(TypeError::ErrorDeAperturaDeArchivo)
                }
                
            }
        };
        resultado_final
    }




}
