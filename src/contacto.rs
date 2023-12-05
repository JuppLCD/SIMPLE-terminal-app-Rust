#[derive(Clone)]
pub struct Contacto {
    nombre: String,
    apellido: String,
    telefono: String,
    email: String,
}

#[derive(Debug)]
pub enum ContactoPropiedad {
    Nombre,
    Apellido,
    Telefono,
    Email,
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

    pub fn es_igual(&self, otro: &Self) -> bool {
        self.apellido == otro.apellido
            && self.nombre == otro.nombre
            && self.telefono == otro.telefono
            && self.email == otro.email
    }

    pub fn editar(&mut self, nombre: String, apellido: String, telefono: String, email: String) {
        self.nombre = nombre;
        self.apellido = apellido;
        self.telefono = telefono;
        self.email = email;
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

pub fn contacto_filtrar_por_propiedad(
    contactos: &Vec<Contacto>,
    propiedad: ContactoPropiedad,
    valor: String,
) -> Vec<Contacto> {
    contactos
        .iter()
        .filter_map(|contacto| match propiedad {
            ContactoPropiedad::Nombre
                if contacto
                    .nombre
                    .to_lowercase()
                    .contains(valor.to_lowercase().as_str()) =>
            {
                Some(contacto.clone())
            }
            ContactoPropiedad::Apellido
                if contacto
                    .apellido
                    .to_lowercase()
                    .contains(valor.to_lowercase().as_str()) =>
            {
                Some(contacto.clone())
            }
            ContactoPropiedad::Telefono
                if contacto
                    .telefono
                    .to_lowercase()
                    .contains(valor.to_lowercase().as_str()) =>
            {
                Some(contacto.clone())
            }
            ContactoPropiedad::Email
                if contacto
                    .email
                    .to_lowercase()
                    .contains(valor.to_lowercase().as_str()) =>
            {
                Some(contacto.clone())
            }
            _ => None,
        })
        .collect()
}
