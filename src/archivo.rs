use crate::errors::{catch, TypeError};
use std::{
    fs::{self},
    io::ErrorKind,
};

pub struct FileHandler {
    path: String,
    tablero: [[char; 8]; 8],
}

impl FileHandler {
    pub fn new(x: Vec<String>) -> Self {
        if x.len() != 2 {
            catch(TypeError::CantidadDeArgumentosIngresadosIncorrecta);
        }
        let pathfile = format!("{}{}", "src/".to_owned(), x[1]);
        FileHandler {
            path: pathfile,
            tablero: [[' '; 8]; 8],
        }
    }

    pub fn leer(&mut self) -> Result<(), TypeError> {
        let contenido = self.leer_archivo_completo(self.path.as_str())?;

        self.contenido_archivo_es_correcto(&contenido)?;

        for (indice_linea, linea) in contenido.lines().enumerate() {
            for (indice_letra, letra) in linea.split(' ').enumerate() {
                self.tablero[indice_linea][indice_letra] = letra.chars().next().unwrap_or_default();
            }
        }
        Ok(())
    }

    fn leer_archivo_completo(&self, filepath: &str) -> Result<String, TypeError> {
        let archivo_resultado = fs::read_to_string(filepath);
        match archivo_resultado {
            Ok(archivo) => Ok(archivo),
            Err(error_obtenido) => {
                if error_obtenido.kind() == ErrorKind::NotFound {
                    Err(TypeError::NombreDeArchivoInvalido)
                } else {
                    Err(TypeError::AperturaDeArchivoInvalida)
                }
            }
        }
    }

    //1 validacion:
    //debe haber siempre 7 " "   y ademas 6,7,8 "_"    OK
    //ademas,iterar y verificar que los espacios esten en las posiciones impares de iteracion
    //ademas, ver que sean 8 iteraciones OK
    //DEBE HAber un total de 62 guiones -> hay mas de 2 piezas en el tablero OK
    fn contenido_archivo_es_correcto(&self, contenido_archivo: &str) -> Result<(), TypeError> {
        if contenido_archivo.lines().count() != 8 {
            Err(TypeError::TamanioDeTableroIncorrecto)
        } else {
            for linea in contenido_archivo.lines() {
                if linea.matches(' ').count() != 7 {
                    return Err(TypeError::ArchivoConFormatoDeEspaciosIncorrecta);
                } else if linea.matches('_').count() < 6 || linea.matches('_').count() > 8 {
                    return Err(TypeError::ArchivoConCantidadDeCasillerosVaciosIncorrecta);
                } else if contenido_archivo.matches('_').count() != 62 {
                    return Err(TypeError::CantidadDePiezasIncorrecta);
                }

                for (indice_letra, letra) in linea.char_indices() {
                    if indice_letra % 2 != 0 && letra != ' ' {
                        return Err(TypeError::ArchivoConFormatoDeEspaciosImparesIncorrecta);
                    }
                }
            }
            Ok(())
        }
    }

    pub fn dar_tablero_procesado(self) -> [[char; 8]; 8] {
        self.tablero
    }
}
