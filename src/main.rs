use crate::opciones_agenda_contacto::opciones::AgendaContactosOpciones;
use crate::opciones_agenda_contacto::{
    agregar::agregar_contacto, buscar::buscar_contacto, editar::editar_contacto,
    eliminar::eliminar_contacto, mostrar_todos::mostrar_todos_contactos,
};

pub mod opciones_agenda_contacto;

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
    let contactos = vec![contacto1, contacto2];

    loop {
        imprimir_opciones_agenda();

        let op = obtener_opcion_valida(0, 5);

        match AgendaContactosOpciones::new(op) {
            AgendaContactosOpciones::MostrarTodos => mostrar_todos_contactos(&contactos),
            AgendaContactosOpciones::Buscar => buscar_contacto(&contactos),
            AgendaContactosOpciones::Agregar => agregar_contacto(),
            AgendaContactosOpciones::Editar => editar_contacto(),
            AgendaContactosOpciones::Eliminar => eliminar_contacto(),
            AgendaContactosOpciones::Salir => break,
        }
    }

    imprimir_msg_salir_agenda()
}
