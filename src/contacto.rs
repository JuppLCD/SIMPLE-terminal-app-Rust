use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Contacto {
    nombre: String,
    apellido: String,
    telefono: String,
    email: String,
    pub id: String,
}

#[derive(Debug)]
pub enum ContactoPropiedad {
    Id,
    Nombre,
    Apellido,
    Telefono,
    Email,
}

// Fn estaticas
impl Contacto {
    pub fn nuevo(
        nombre: String,
        apellido: String,
        telefono: String,
        email: String,
        id: String,
    ) -> Self {
        Contacto {
            id,
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

    pub fn editar(&mut self, nombre: String, apellido: String, telefono: String, email: String) {
        self.nombre = nombre;
        self.apellido = apellido;
        self.telefono = telefono;
        self.email = email;
    }
}

impl ContactoPropiedad {
    pub fn nuevo(opcion: u8) -> ContactoPropiedad {
        match opcion {
            1 => ContactoPropiedad::Nombre,
            2 => ContactoPropiedad::Apellido,
            3 => ContactoPropiedad::Telefono,
            4 => ContactoPropiedad::Email,
            _ => ContactoPropiedad::Id,
        }
    }
}

pub fn contacto_imprimir_vector(contactos: &[Contacto]) {
    for (i, cont) in contactos.iter().enumerate() {
        println!("\nContacto N°{}", i + 1);
        println!("-------------------------------");
        cont.imprimir()
    }
    println!("-------------------------------");
}

pub fn contacto_filtrar_por_propiedad(
    contactos: &[Contacto],
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
            ContactoPropiedad::Id
                if contacto
                    .id
                    .to_lowercase()
                    .contains(valor.to_lowercase().as_str()) =>
            {
                Some(contacto.clone())
            }
            _ => None,
        })
        .collect()
}
