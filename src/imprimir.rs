pub fn limpiar_terminal() {
    println!("\n\n\n");
    // std::process::Command::new("clear").status().unwrap(); // Linea que puede ser comentada o descomentada para debugear
}

pub fn imprimir_opciones_agenda() {
    limpiar_terminal();

    println!("        AGENDA DE CONTACTO");
    println!("Seleccione alguna de las siguientes opciones:");
    println!("1) Mostrar todos los contactos agendados.");
    println!("2) Buscar contacto.");
    println!("3) Agregar un nuevo contacto.");
    println!("4) Editar contacto.");
    println!("5) Eliminar contacto.");

    println!("0) Salir de la agenda.");
}

pub fn imprimir_msg_salir_agenda() {
    println!("\nCerrando la agenda de contactos.");
}

pub fn imprimir_cabecera_mostrar_todos_contactos() {
    limpiar_terminal();

    println!("########### Contactos ###########");
    println!("Estos son todos tus contactos:");
}

pub fn imprimir_cabecera_buscar_contacto() {
    limpiar_terminal();

    println!("########### Busqueda de contacto ###########");
    println!("Por cual de las siguientes propiedades deseas buscar a tu contacto?");
    println!("1) Nombre.");
    println!("2) Apellido.");
    println!("3) Numero de telefono.");
    println!("4) Email.");

    println!("0) Ir al menu.");
}

pub fn imprimir_cabecera_alamcenar_contacto() {
    limpiar_terminal();

    println!("########### Añadiendo nuevo contacto ###########");
    println!("Ingrese los datos del contacto: ");
}

pub fn imprimir_opciones_alamcenar_contacto() {
    println!("\nDesea: ");
    println!("1) Almacenar.");
    println!("2) Reacer.");
    println!("3) Almacenar y crear nuevo contacto.");

    println!("0) Descartar contacto e ir al menu.");
}

pub fn imprimir_cabecera_modificar_contacto() {
    limpiar_terminal();

    println!("########### Edicion de contacto ###########");
    println!("Por cual de las siguientes propiedades deseas buscar al contacto a modificar?");
    println!("1) Nombre.");
    println!("2) Apellido.");
    println!("3) Numero de telefono.");
    println!("4) Email.");

    println!("0) Ir al menu.");
}

pub fn imprimir_cabecera_eliminar_contacto() {
    limpiar_terminal();

    println!("########### Eliminar contacto ###########");
    println!("Por cual de las siguientes propiedades deseas eliminar a tu contacto?");
    println!("1) Nombre.");
    println!("2) Apellido.");
    println!("3) Numero de telefono.");
    println!("4) Email.");

    println!("0) Ir al menu.");
}
