use std::io;

pub fn limpiar_terminal() {
    println!("\n\n\n");
    // std::process::Command::new("clear").status().unwrap(); // Linea que puede ser comentada o descomentada para debugear
}

pub fn pausar_terminal() {
    println!("Presiona ENTER para continuar.....");

    let mut pause = String::new();

    io::stdin()
        .read_line(&mut pause)
        .expect("Fallo al leer la linea");
}
