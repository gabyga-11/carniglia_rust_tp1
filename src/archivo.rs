use crate::errors::{catch, TypeError};
use std::{
    fs::{self},
    io::ErrorKind,
};

/// Es el encargado de realizar todo el procesamiento inicial del archivo,
/// para obtener un tablero de 8 x 8 de caracteres válido.
/// 
/// Contiene el path desde src/*.txt con el nombre del archivo input. 
/// Junto con una matriz de chars de 8 x 8, que permite cargar los datos del archivo.
#[derive(PartialEq, Debug)]
pub struct FileHandler {
    path: String,
    tablero: [[char; 8]; 8],
}

impl FileHandler {
    /// Se le pasan los argumentos de consola por parametro.
    /// Retorna un filehandler válido
    /// ### Errores
    /// Si se ingresa uno o más parametros, se cortará la ejecución y retornará
    /// por pantalla un error por cantidad de argumentos invalida
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

    /// Realiza todo el procesamiento de lectura. Llama a la función que lee y a otra que analiza el contenido.
    /// Luego, carga cada caracter en la matriz 8 x 8, que es atributo de la clase archivo
    /// ### Errores
    /// Si ocurre algún error en la lectura o analisis del archivo, el error se propagará y será devuelto por esta función
    pub fn leer(&mut self) -> Result<(), TypeError> {
        let contenido = self.abrir_y_cargar_contenido(self.path.as_str())?;
        self.analizar_contenido_archivo(&contenido)?;

        for (indice_linea, linea) in contenido.lines().enumerate() {
            for (indice_letra, letra) in linea.split(' ').enumerate() {
                self.tablero[indice_linea][indice_letra] = letra.chars().next().unwrap_or_default();
            }
        }
        Ok(())
    }

    /// Se le pasa el path completo. Abre y lee todo el contenido del archivo, para luego retornarlo.
    /// ### Errores
    /// Si ocurre que no encuentra al archivo, devolvera TypeError NombreDeArchivoInvalido.
    /// Si por culaquier otro motivo no puede abrirse el archivo, devolvera TypeError AperturaDeArchivoInvalida
    fn abrir_y_cargar_contenido(&self, filepath: &str) -> Result<String, TypeError> {
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
    
    /// Se le pasa el contenido completo del archivo. Realiza un análisis global y linea por linea para identificar
    /// distintos posible errores de formato del archivo. Si es Ok, retorna vacío.
    /// 
    /// ## Errores de analisis global 
    /// Si la cantidad de lineas no es 8, retorna TypeError TamanioDeTableroIncorrecto.
    /// 
    /// Si no hay 62 guiones (luego de descartar otras posibilidades), retorna TypeError TamanioDeTableroIncorrecto.
    /// 
    /// Si por culaquier otro motivo no puede abrirse el archivo, devolvera TypeError CantidadDePiezasIncorrecta.
    /// 
    /// ## Errores de analisis linea por linea
    /// 
    /// Con encontrar una linea que cumpla alguna de estas condiciones, ya retorna el error.
    /// 
    /// Si la cantidad de espacios no es 7, retorna TypeError ArchivoConFormatoDeEspaciosIncorrecta.
    /// 
    /// Si la cantidad de guiones es meno que 6 o mayor de 8, retorna TypeError ArchivoConCantidadDeCasillerosVaciosIncorrecta.
    /// 
    /// Si se encuentra algun caracter en la posicion que le corresponde a un espacio, retorna TypeError ArchivoConCantidadDeCasillerosVaciosIncorrecta
    fn analizar_contenido_archivo(&self, contenido_archivo: &str) -> Result<(), TypeError> {
        if contenido_archivo.lines().count() != 8 {
            Err(TypeError::TamanioDeTableroIncorrecto)
        } else {
            for linea in contenido_archivo.lines() {
                if linea.matches(' ').count() != 7 {
                    return Err(TypeError::ArchivoConFormatoDeEspaciosIncorrecta);
                } else if linea.matches('_').count() < 6 || linea.matches('_').count() > 8 {
                    return Err(TypeError::ArchivoConCantidadDeCasillerosVaciosIncorrecta);
                }
                for (indice_letra, letra) in linea.char_indices() {
                    if indice_letra % 2 != 0 && letra != ' ' {
                        return Err(TypeError::ArchivoConFormatoDeEspaciosImparesIncorrecta);
                    }
                }
            }
            if contenido_archivo.matches('_').count() != 62 {
                return Err(TypeError::CantidadDePiezasIncorrecta);
            }
            Ok(())
        }
    }
    ////Retorna el atributo tablero
    pub fn dar_tablero_procesado(self) -> [[char; 8]; 8] {
        self.tablero
    }
}

#[cfg(test)]
mod tests {
    use crate::{archivo::FileHandler, errors::TypeError};

    #[test]
    fn test_new() {
        let args: Vec<String> = [
            String::from("target/debug/rust_tp1_carniglia"),
            String::from("tablero.txt"),
        ]
        .to_vec();
        let fh_ok = FileHandler {
            path: String::from("src/tablero.txt"),
            tablero: [[' '; 8]; 8],
        };
        assert_eq!(FileHandler::new(args), fh_ok);
    }

    #[test]
    fn test_leer_ok() {
        let mut fh_ok = FileHandler {
            path: String::from("src/table.txt"),
            tablero: [[' '; 8]; 8],
        };
        assert_eq!(fh_ok.leer(), Ok(()));
    }

    #[test]
    fn test_leer_err() {
        let mut fh_err = FileHandler {
            path: String::from("src/tablerito.txt"),
            tablero: [[' '; 8]; 8],
        };
        assert_eq!(fh_err.leer(), Err(TypeError::NombreDeArchivoInvalido));
    }

    #[test]
    fn test_contenido_archivo_es_correcto() {
        let fh_ok = definir_fh();
        let contenido = "_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ P _ _ _\n_ _ _ _ _ p _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _";
        assert_eq!(fh_ok.analizar_contenido_archivo(contenido), Ok(()));
    }
    #[test]
    fn test_contenido_archivo_linea_menos(){
        let fh_ok = definir_fh();
        let contenido = "_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ P _ _ _\n_ _ _ _ _ p _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _";
        assert_eq!(
            fh_ok.analizar_contenido_archivo(contenido),
            Err(TypeError::TamanioDeTableroIncorrecto)
        );
    }
    #[test]
    fn test_contenido_archivo_linea_mas(){
        let fh_ok = definir_fh();
        let contenido = "_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ P _ _ _\n_ _ _ _ _ p _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _";
        assert_eq!(
            fh_ok.analizar_contenido_archivo(contenido),
            Err(TypeError::TamanioDeTableroIncorrecto)
        );
    }
    #[test]
    fn test_contenido_archivo_espacio_de_mas(){
        let fh_ok = definir_fh();
        let contenido = "_ _ _ _ _ _ _ _\n_ _ _ _ _  _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ P _ _ _\n_ _ _ _ _ p _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _";
        assert_eq!(
            fh_ok.analizar_contenido_archivo(contenido),
            Err(TypeError::ArchivoConFormatoDeEspaciosIncorrecta)
        );
    }
    #[test]
    fn test_contenido_archivo_cantidad_vacio_por_linea_incorrecta(){
        let fh_ok = definir_fh();
        let contenido = "_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _a_\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ P _ _ _\n_ _ _ _ _ p _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _";
        assert_eq!(
            fh_ok.analizar_contenido_archivo(contenido),
            Err(TypeError::ArchivoConCantidadDeCasillerosVaciosIncorrecta)
        );
    }
    #[test]
    fn test_contenido_archivo_cantidad_piezas_incorrecta(){
        let fh_ok = definir_fh();
        let contenido = "_ _ _ _ r _ _ _\n_ _ _ _ a _ _ _\np _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ P _ _ _\n_ _ _ _ _ p _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _";
        assert_eq!(
            fh_ok.analizar_contenido_archivo(contenido),
            Err(TypeError::CantidadDePiezasIncorrecta)
        );
    }
    #[test]
    fn test_contenido_archivo_string_aleatorio(){
        let fh_ok = definir_fh();
        let contenido = "_ _ _ _ r _ _qweriuywqegri _\n_ _ _ _ a _ _ _\np _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ P _ _ _\n_ _ _ _ _ p _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _";
        assert_eq!(
            fh_ok.analizar_contenido_archivo(contenido),
            Err(TypeError::ArchivoConFormatoDeEspaciosImparesIncorrecta)
        );
    }

    fn definir_fh() -> FileHandler{
        FileHandler {
            path: String::from(""),
            tablero: [[' '; 8]; 8],
        }
    }
}
