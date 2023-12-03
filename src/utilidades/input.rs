use std::io;

pub fn obtener_opcion_valida(primera_opcion: u8, ultima_opcion: u8) -> u8 {
    return loop {
        println!("Opcion [{primera_opcion}-{ultima_opcion}]: ");

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
        println!("Selecione dentro del rango dado\n");
    };
}
