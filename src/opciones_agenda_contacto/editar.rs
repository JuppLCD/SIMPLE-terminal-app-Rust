use crate::contacto::{
    contacto_filtrar_por_propiedad, contacto_imprimir_vector, Contacto, ContactoPropiedad,
};

use crate::imprimir::{imprimir_cabecera_modificar_contacto, limpiar_terminal};

use crate::utilidades::input::{obtener_opcion_valida, obtener_texto};
use crate::utilidades::terminal::pausar_terminal;

pub fn editar_contacto(contactos: &mut Vec<Contacto>) {
    if contactos.is_empty() {
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
            contacto_filtrar_por_propiedad(contactos, propiedad_buscar, valor_buscar);

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
            let index_contacto_editar = obtener_opcion_valida(1, contactos_filtrados.len() as u8);
            let index_contacto_editar = index_contacto_editar - 1;

            let contacto_a_editar = &contactos_filtrados[index_contacto_editar as usize];

            // Obtengo los nuevos valores del contacto
            println!("\n\nColoque los nuevos valores para el contacto:");
            let nombre = obtener_texto("- Nombre: ".to_string());
            let apellido = obtener_texto("- Apellido: ".to_string());
            let telefono = obtener_texto("- Numero de telefono: ".to_string());
            let email = obtener_texto("- Email: ".to_string());

            if let Some(cont) = contactos.iter_mut().find(|c| c.es_igual(contacto_a_editar)) {
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
