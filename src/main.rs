use std::io;

use self::opciones_agenda_contacto::{
    agregar::agregar_contacto, buscar::buscar_contacto, editar::editar_contacto,
    eliminar::eliminar_contacto, mostrar_todos::mostrar_todos_contactos,
};

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

fn imprimir_opciones_agenda() {
    println!("        AGENDA DE CONTACTO");
    println!("Seleccione alguna de las siguientes opciones:");
    println!("1) Mostrar todos los contactos agendados.");
    println!("2) Buscar contacto.");
    println!("3) Agregar un nuevo contacto.");
    println!("4) Editar contacto.");
    println!("5) Eliminar contacto.");

    println!("0) Salir de la agenda.");
}

fn obtener_opcion_valida(primera_opcion: u8, ultima_opcion: u8) -> u8 {
    return loop {
        println!("Opcion ({primera_opcion}-{ultima_opcion}): ");

        let mut op_usuario = String::new();

        io::stdin()
            .read_line(&mut op_usuario)
            .expect("Fallo al leer la linea");

        if let Ok(num) = op_usuario.trim().parse::<u8>() {
            if primera_opcion <= num && num <= ultima_opcion {
                break num;
            }
        };

        println!("Opcion no valida!!");
    };
}

enum AgendaContactosOpciones {
    MostrarTodos,
    Buscar,
    Agregar,
    Editar,
    Eliminar,
    Salir,
}

impl AgendaContactosOpciones {
    fn new(opcion: u8) -> AgendaContactosOpciones {
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

fn pausar_terminal() {
    println!("Presiona ENTER para continuar.....");

    let mut pause = String::new();

    io::stdin()
        .read_line(&mut pause)
        .expect("Fallo al leer la linea");
}

mod opciones_agenda_contacto {
    use super::pausar_terminal;
    pub mod mostrar_todos {
        pub fn mostrar_todos_contactos() {
            println!("Opcion MostrarTodos");
            super::pausar_terminal();
        }
    }
    pub mod buscar {
        pub fn buscar_contacto() {
            println!("Opcion Buscar");
            super::pausar_terminal();
        }
    }
    pub mod agregar {
        pub fn agregar_contacto() {
            println!("Opcion Agregar");
            super::pausar_terminal();
        }
    }
    pub mod editar {
        pub fn editar_contacto() {
            println!("Opcion Editar");
            super::pausar_terminal();
        }
    }
    pub mod eliminar {
        pub fn eliminar_contacto() {
            println!("Opcion Eliminar");
            super::pausar_terminal();
        }
    }
}
