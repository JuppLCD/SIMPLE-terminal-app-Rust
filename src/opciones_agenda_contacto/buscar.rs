use crate::contacto::{
    contacto_filtrar_por_propiedad, contacto_imprimir_vector, Contacto, ContactoPropiedad,
};
use crate::imprimir::imprimir_cabecera_buscar_contacto;

use crate::utilidades::input::{obtener_opcion_valida, obtener_texto};
use crate::utilidades::terminal::pausar_terminal;

pub fn buscar_contacto(contactos: &Vec<Contacto>) {
    if contactos.len() == 0 {
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
            contacto_filtrar_por_propiedad(contactos, propiedad_buscar, valor_buscar);

        if contactos_filtrados.len() == 0 {
            println!("\n\nNo se ha encontrado nigun contacto con dicho valor.\n");
        } else {
            println!("\n\nSe a encontrado los siguientes contactos:");
            contacto_imprimir_vector(&contactos_filtrados);
        }
    }

    pausar_terminal();
}
