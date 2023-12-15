use crate::{
    contacto::{
        contacto_filtrar_por_propiedad, contacto_imprimir_vector, Contacto, ContactoPropiedad,
    },
    imprimir::{
        imprimir_cabecera_alamcenar_contacto, imprimir_cabecera_buscar_contacto,
        imprimir_cabecera_eliminar_contacto, imprimir_cabecera_modificar_contacto,
        imprimir_cabecera_mostrar_todos_contactos, imprimir_opciones_alamcenar_contacto,
    },
    utilidades::{
        input::{obtener_opcion_valida, obtener_texto},
        terminal::{limpiar_terminal, pausar_terminal},
    },
};

pub struct AgendaContacto {
    pub contactos: Vec<Contacto>,
}

impl AgendaContacto {
    pub fn mostrar_todos(&self) {
        imprimir_cabecera_mostrar_todos_contactos();
        contacto_imprimir_vector(&self.contactos);
        pausar_terminal();
    }

    pub fn buscar(&self) {
        if self.contactos.is_empty() {
            println!("No se a registrado ningun contacto en la agenda.");
        } else {
            imprimir_cabecera_buscar_contacto();
            let op = obtener_opcion_valida(0, 4);

            if 0 == op {
                return; // Volver a menu principal
            }

            let propiedad_buscar = ContactoPropiedad::nuevo(op);

            let valor_buscar = obtener_texto(format!(
                "\nIngrese el '{:?}' del contacto a buscar: ",
                propiedad_buscar
            ));

            let contactos_filtrados =
                contacto_filtrar_por_propiedad(&self.contactos, propiedad_buscar, valor_buscar);

            if contactos_filtrados.is_empty() {
                println!("\n\nNo se ha encontrado nigun contacto con dicho valor.\n");
            } else {
                println!("\n\nSe a encontrado los siguientes contactos:");
                contacto_imprimir_vector(&contactos_filtrados);
            }
        }

        pausar_terminal();
    }

    pub fn agregar(&mut self) {
        loop {
            imprimir_cabecera_alamcenar_contacto();

            let nombre = obtener_texto("- Nombre: ".to_string());
            let apellido = obtener_texto("- Apellido: ".to_string());
            let telefono = obtener_texto("- Numero de telefono: ".to_string());
            let email = obtener_texto("- Email: ".to_string());

            let nuevo_contacto = Contacto::nuevo(nombre, apellido, telefono, email);

            limpiar_terminal();
            println!("El nuevo contacto queda con los siguientes valores:");
            nuevo_contacto.imprimir();

            imprimir_opciones_alamcenar_contacto();
            let op = obtener_opcion_valida(0, 3);

            match OpcionesAgregarContacto::nuevo(op) {
                OpcionesAgregarContacto::AlmacenarYSalir => {
                    self.contactos.push(nuevo_contacto);
                    return;
                }
                OpcionesAgregarContacto::Rehacer => continue,
                OpcionesAgregarContacto::AlmacenarYContinuar => self.contactos.push(nuevo_contacto),
                OpcionesAgregarContacto::Salir => return,
            }
        }
    }

    pub fn editar(&mut self) {
        if self.contactos.is_empty() {
            println!("No se a registrado ningun contacto en la agenda.");
        } else {
            imprimir_cabecera_modificar_contacto();
            let op = obtener_opcion_valida(0, 4);

            if 0 == op {
                return; // Volver a menu principal
            }

            let propiedad_buscar = ContactoPropiedad::nuevo(op);

            let valor_buscar = obtener_texto(format!(
                "\nIngrese el '{:?}' del contacto a editar: ",
                propiedad_buscar
            ));

            let contactos_filtrados =
                contacto_filtrar_por_propiedad(&self.contactos, propiedad_buscar, valor_buscar);

            if contactos_filtrados.is_empty() {
                println!("\n\nNo se ha encontrado nigun contacto con dicho valor.\n");
            } else {
                println!("\n\nSe a encontrado los siguientes contactos:");
                contacto_imprimir_vector(&contactos_filtrados);

                println!(
                    "\nCual contacto deseas modificar? Contacto N°(1-{})\n",
                    contactos_filtrados.len()
                );

                // Obtener el indice del contacto a modificar
                let index_contacto_editar =
                    obtener_opcion_valida(1, contactos_filtrados.len() as u8);
                let index_contacto_editar = index_contacto_editar - 1;

                let contacto_a_editar = &contactos_filtrados[index_contacto_editar as usize];

                // Obtengo los nuevos valores del contacto
                println!("\n\nColoque los nuevos valores para el contacto:");
                let nombre = obtener_texto("- Nombre: ".to_string());
                let apellido = obtener_texto("- Apellido: ".to_string());
                let telefono = obtener_texto("- Numero de telefono: ".to_string());
                let email = obtener_texto("- Email: ".to_string());

                if let Some(cont) = self
                    .contactos
                    .iter_mut()
                    .find(|c| c.es_igual(contacto_a_editar))
                {
                    cont.editar(nombre, apellido, telefono, email);

                    limpiar_terminal();
                    println!("\nEl contacto se modifico con exito!!");
                    println!("El contacto modificado queda con los siguientes valores:");
                    cont.imprimir();
                    println!();
                } else {
                    println!("\nEl contacto no se pudo modificar!!");
                }
            }
        }

        pausar_terminal();
    }

    pub fn eliminar(&mut self) {
        if self.contactos.is_empty() {
            println!("No se a registrado ningun contacto en la agenda.");
        } else {
            imprimir_cabecera_eliminar_contacto();
            let op = obtener_opcion_valida(0, 4);

            if 0 == op {
                return; // Volver a menu principal
            }

            let propiedad_buscar = ContactoPropiedad::nuevo(op);

            let valor_buscar = obtener_texto(format!(
                "\nIngrese el '{:?}' del contacto a eliminar: ",
                propiedad_buscar
            ));

            let contactos_filtrados =
                contacto_filtrar_por_propiedad(&self.contactos, propiedad_buscar, valor_buscar);

            if contactos_filtrados.is_empty() {
                println!("\n\nNo se ha encontrado nigun contacto con dicho valor.\n");
            } else {
                println!("\n\nSe a encontrado los siguientes contactos:");
                contacto_imprimir_vector(&contactos_filtrados);

                println!(
                    "\nCual contacto deseas eliminar? Contacto N°(1-{})\n",
                    contactos_filtrados.len()
                );

                // Obtener el indice del contacto a modificar
                let index_contacto_eliminar =
                    obtener_opcion_valida(1, contactos_filtrados.len() as u8);
                let index_contacto_eliminar = (index_contacto_eliminar - 1) as usize;

                let contacto_a_eliminar = &contactos_filtrados[index_contacto_eliminar];

                if let Some(index) = self
                    .contactos
                    .iter()
                    .position(|c| c.es_igual(contacto_a_eliminar))
                {
                    limpiar_terminal();
                    self.contactos.remove(index);
                    println!("\nEl contacto se elimino con exito!!");
                } else {
                    println!("\nEl contacto no se pudo eliminar!!");
                }
            }
        }
        pausar_terminal();
    }
}

enum OpcionesAgregarContacto {
    AlmacenarYSalir,
    Rehacer,
    AlmacenarYContinuar,
    Salir,
}

impl OpcionesAgregarContacto {
    fn nuevo(opcion: u8) -> OpcionesAgregarContacto {
        match opcion {
            1 => OpcionesAgregarContacto::AlmacenarYSalir,
            2 => OpcionesAgregarContacto::Rehacer,
            3 => OpcionesAgregarContacto::AlmacenarYContinuar,
            _ => OpcionesAgregarContacto::Salir,
        }
    }
}

pub enum AgendaContactosOpciones {
    MostrarTodos,
    Buscar,
    Agregar,
    Editar,
    Eliminar,
    Salir,
}

impl AgendaContactosOpciones {
    pub fn nuevo(opcion: u8) -> AgendaContactosOpciones {
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
