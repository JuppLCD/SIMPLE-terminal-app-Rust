use crate::contacto::Contacto;
use crate::imprimir::{
    imprimir_cabecera_alamcenar_contacto, imprimir_opciones_alamcenar_contacto, limpiar_terminal,
};
use crate::utilidades::input::{obtener_opcion_valida, obtener_texto};

pub fn agregar_contacto(contactos: &mut Vec<Contacto>) {
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
                contactos.push(nuevo_contacto);
                return;
            }
            OpcionesAgregarContacto::Rehacer => continue,
            OpcionesAgregarContacto::AlmacenarYContinuar => contactos.push(nuevo_contacto),
            OpcionesAgregarContacto::Salir => return,
        }
    }
}

pub enum OpcionesAgregarContacto {
    AlmacenarYSalir,
    Rehacer,
    AlmacenarYContinuar,
    Salir,
}

impl OpcionesAgregarContacto {
    pub fn nuevo(opcion: u8) -> OpcionesAgregarContacto {
        match opcion {
            1 => OpcionesAgregarContacto::AlmacenarYSalir,
            2 => OpcionesAgregarContacto::Rehacer,
            3 => OpcionesAgregarContacto::AlmacenarYContinuar,
            _ => OpcionesAgregarContacto::Salir,
        }
    }
}
