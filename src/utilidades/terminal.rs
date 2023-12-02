use std::io;

pub fn pausar_terminal() {
    println!("Presiona ENTER para continuar.....");

    let mut pause = String::new();

    io::stdin()
        .read_line(&mut pause)
        .expect("Fallo al leer la linea");
}
