use agenda_contacto::{AgendaContacto, AgendaContactosOpciones};
mod agenda_contacto;

use crate::utilidades::input::obtener_opcion_valida;
pub mod utilidades;

use crate::imprimir::{imprimir_msg_salir_agenda, imprimir_opciones_agenda};
pub mod imprimir;

use crate::contacto::Contacto;
pub mod contacto;

fn main() {
    // Contactos de prueba
    let contacto1 = Contacto::nuevo(
        "Lorenzo".to_string(),
        "Canovas".to_string(),
        "846857".to_string(),
        "lorenzo@gmail.com".to_string(),
    );
    let contacto2 = Contacto::nuevo(
        "Pepe".to_string(),
        "Gonsalez".to_string(),
        "65436".to_string(),
        "pepe@gmail.com".to_string(),
    );

    let mut agenda = AgendaContacto {
        contactos: vec![contacto1, contacto2],
    };

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
