use crate::imprimir::{imprimir_cabecera_alamcenar_contacto, imprimir_opciones_alamcenar_contacto};
use crate::utilidades::terminal::pausar_terminal;

pub fn agregar_contacto() {
    imprimir_cabecera_alamcenar_contacto();
    // Todo: Realizar la logica de obtener los datos de un contacto
    imprimir_opciones_alamcenar_contacto();
    pausar_terminal();
}
