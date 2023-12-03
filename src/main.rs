use crate::opciones_agenda_contacto::opciones::AgendaContactosOpciones;
use crate::opciones_agenda_contacto::{
    agregar::agregar_contacto, buscar::buscar_contacto, editar::editar_contacto,
    eliminar::eliminar_contacto, mostrar_todos::mostrar_todos_contactos,
};

pub mod opciones_agenda_contacto;

use crate::utilidades::input::obtener_opcion_valida;
pub mod utilidades;

use crate::imprimir::imprimir_opciones_agenda;
pub mod imprimir;

fn main() {
    loop {
        // std::process::Command::new("clear").status().unwrap();
        imprimir_opciones_agenda();

        let op = obtener_opcion_valida(0, 5);

        match AgendaContactosOpciones::new(op) {
            AgendaContactosOpciones::MostrarTodos => mostrar_todos_contactos(),
            AgendaContactosOpciones::Buscar => buscar_contacto(),
            AgendaContactosOpciones::Agregar => agregar_contacto(),
            AgendaContactosOpciones::Editar => editar_contacto(),
            AgendaContactosOpciones::Eliminar => eliminar_contacto(),
            AgendaContactosOpciones::Salir => break,
        }
    }

    println!("Opcion Salir")
}
