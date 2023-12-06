use crate::contacto::{
    contacto_filtrar_por_propiedad, contacto_imprimir_vector, Contacto, ContactoPropiedad,
};
use crate::imprimir::{imprimir_cabecera_eliminar_contacto, limpiar_terminal};
use crate::utilidades::input::{obtener_opcion_valida, obtener_texto};
use crate::utilidades::terminal::pausar_terminal;

pub fn eliminar_contacto(contactos: &mut Vec<Contacto>) {
    if contactos.len() == 0 {
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
            contacto_filtrar_por_propiedad(contactos, propiedad_buscar, valor_buscar);

        if contactos_filtrados.len() == 0 {
            println!("\n\nNo se ha encontrado nigun contacto con dicho valor.\n");
        } else {
            println!("\n\nSe a encontrado los siguientes contactos:");
            contacto_imprimir_vector(&contactos_filtrados);

            println!(
                "\nCual contacto deseas eliminar? Contacto N°(1-{})\n",
                contactos_filtrados.len()
            );

            // Obtener el indice del contacto a modificar
            let index_contacto_eliminar = obtener_opcion_valida(1, contactos_filtrados.len() as u8);
            let index_contacto_eliminar = (index_contacto_eliminar - 1) as usize;

            let contacto_a_eliminar = &contactos_filtrados[index_contacto_eliminar];

            if let Some(index) = contactos
                .iter()
                .position(|c| c.es_igual(contacto_a_eliminar))
            {
                limpiar_terminal();
                contactos.remove(index);
                println!("\nEl contacto se elimino con exito!!");
            } else {
                println!("\nEl contacto no se pudo eliminar!!");
            }
        }
    }
    pausar_terminal();
}
