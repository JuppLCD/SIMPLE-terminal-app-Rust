use agenda_contacto::{AgendaContacto, AgendaContactosOpciones};
mod agenda_contacto;

use crate::utilidades::input::obtener_opcion_valida;
pub mod utilidades;

use crate::imprimir::{imprimir_msg_salir_agenda, imprimir_opciones_agenda};
pub mod imprimir;

pub mod contacto;

pub mod models;

fn main() {
    let mut agenda = AgendaContacto::iniciar();

    loop {
        imprimir_opciones_agenda();

        let op = obtener_opcion_valida(0, 5);

        match AgendaContactosOpciones::nuevo(op) {
            AgendaContactosOpciones::MostrarTodos => agenda.mostrar_todos(),
            AgendaContactosOpciones::Buscar => agenda.buscar(),
            AgendaContactosOpciones::Agregar => agenda.agregar(),
            AgendaContactosOpciones::Editar => agenda.editar(),
            AgendaContactosOpciones::Eliminar => agenda.eliminar(),
            AgendaContactosOpciones::Salir => break,
        }
    }

    imprimir_msg_salir_agenda()
}
