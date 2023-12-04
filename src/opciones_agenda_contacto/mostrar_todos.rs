use crate::contacto::{contacto_imprimir_vector, Contacto};
use crate::imprimir::imprimir_cabecera_mostrar_todos_contactos;
use crate::utilidades::terminal::pausar_terminal;

pub fn mostrar_todos_contactos(contactos: &Vec<Contacto>) {
    imprimir_cabecera_mostrar_todos_contactos();
    contacto_imprimir_vector(contactos);
    pausar_terminal();
}
