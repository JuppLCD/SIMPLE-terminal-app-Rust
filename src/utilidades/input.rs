use std::io;

pub fn obtener_opcion_valida(primera_opcion: u8, ultima_opcion: u8) -> u8 {
    return loop {
        let op_usuario = obtener_texto(format!("Opcion [{primera_opcion}-{ultima_opcion}]: "));

        if let Ok(num) = op_usuario.trim().parse::<u8>() {
            if primera_opcion <= num && num <= ultima_opcion {
                break num;
            }
        };

        println!("Opcion no valida!!");
        println!("Selecione dentro del rango dado\n");
    };
}

pub fn obtener_texto(texto: String) -> String {
    println!("{texto}");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Fallo al leer la linea");

    input.trim().to_string()
}
