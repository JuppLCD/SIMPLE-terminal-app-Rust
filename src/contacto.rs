pub struct Contacto {
    nombre: String,
    apellido: String,
    telefono: String,
    email: String,
}

// Fn estaticas
impl Contacto {
    pub fn nuevo(nombre: String, apellido: String, telefono: String, email: String) -> Self {
        Contacto {
            nombre,
            apellido,
            telefono,
            email,
        }
    }
}

// Metodos
impl Contacto {
    pub fn imprimir(&self) {
        println!("Nombre: {}", self.nombre);
        println!("Apellido: {}", self.apellido);
        println!("Tel.: {}", self.telefono);
        println!("Email: {}", self.email);
    }
}

pub fn contacto_imprimir_vector(contactos: &Vec<Contacto>) {
    for (i, cont) in contactos.iter().enumerate() {
        println!("\nContacto N°{}", i + 1);
        println!("-------------------------------");
        cont.imprimir()
    }
    println!("-------------------------------");
}
