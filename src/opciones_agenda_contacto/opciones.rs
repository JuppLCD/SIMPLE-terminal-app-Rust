pub enum AgendaContactosOpciones {
    MostrarTodos,
    Buscar,
    Agregar,
    Editar,
    Eliminar,
    Salir,
}

impl AgendaContactosOpciones {
    pub fn new(opcion: u8) -> AgendaContactosOpciones {
        match opcion {
            1 => AgendaContactosOpciones::MostrarTodos,
            2 => AgendaContactosOpciones::Buscar,
            3 => AgendaContactosOpciones::Agregar,
            4 => AgendaContactosOpciones::Editar,
            5 => AgendaContactosOpciones::Eliminar,
            _ => AgendaContactosOpciones::Salir,
        }
    }
}
