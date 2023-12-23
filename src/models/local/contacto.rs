use std::{error::Error, fs, io::ErrorKind, path};

use uuid::Uuid;

use crate::contacto::{contacto_filtrar_por_propiedad, Contacto, ContactoPropiedad};

const ARCHIVO: &str = "./src/db/contactos.csv";

pub fn leer_archivo_csv_de_contactos() -> Result<Vec<Contacto>, Box<dyn Error>> {
    let file_content = fs::read_to_string(path::Path::new(ARCHIVO));

    if let Err(e) = file_content {
        match e.kind() {
            ErrorKind::NotFound => {
                fs::File::create(path::Path::new(ARCHIVO)).expect("Error al crear el archivo");

                return Ok(Vec::new());
            }
            _ => panic!("Error al abrir el archivo csv"),
        };
    }

    let text = file_content?;
    let mut reader = csv::Reader::from_reader(text.as_bytes());

    let mut contactos = Vec::new();

    for result in reader.records() {
        let record = result?;

        let nombre = record.get(0).unwrap_or_default().to_string();
        let apellido = record.get(1).unwrap_or_default().to_string();
        let telefono = record.get(2).unwrap_or_default().to_string();
        let email = record.get(3).unwrap_or_default().to_string();
        let id = record.get(4).unwrap_or_default().to_string();

        let contacto = Contacto::nuevo(nombre, apellido, telefono, email, id);
        contactos.push(contacto);
    }

    Ok(contactos)
}

pub fn guardar_contactos_como_csv(contactos: &[Contacto]) -> Result<(), Box<dyn Error>> {
    let file = fs::File::create(path::Path::new(ARCHIVO))?;
    let mut writer = csv::Writer::from_writer(file);

    for contacto in contactos {
        writer.serialize(contacto)?;
    }

    writer.flush()?;
    Ok(())
}

type DatosContacto = (String, String, String, String); // (nombre, apellido, telefono, email)

pub struct ModeloContacto;

impl ModeloContacto {
    pub fn get_all() -> Vec<Contacto> {
        leer_archivo_csv_de_contactos().unwrap()
    }

    pub fn get_by(propiedad: ContactoPropiedad, valor: String) -> Vec<Contacto> {
        let contactos = Self::get_all();

        contacto_filtrar_por_propiedad(&contactos, propiedad, valor)
    }

    pub fn store((nombre, apellido, telefono, email): DatosContacto) -> Vec<Contacto> {
        let nuevo_contacto = Contacto::nuevo(
            nombre,
            apellido,
            telefono,
            email,
            Uuid::new_v4().to_string(),
        );

        let mut contactos = Self::get_all();
        contactos.push(nuevo_contacto);

        guardar_contactos_como_csv(&contactos).unwrap();

        contactos
    }

    pub fn update(
        contacto_id: String,
        (nombre, apellido, telefono, email): DatosContacto,
    ) -> Contacto {
        let mut contactos = Self::get_all();

        let contacto_editar = contactos.iter_mut().find(|c| c.id == contacto_id).unwrap();

        contacto_editar.editar(nombre, apellido, telefono, email);

        guardar_contactos_como_csv(&contactos).unwrap();

        contactos
            .iter()
            .find(|c| c.id == contacto_id)
            .unwrap()
            .to_owned()
    }

    pub fn delete(contacto_id: String) -> Vec<Contacto> {
        let mut contactos = Self::get_all();

        let index = contactos.iter().position(|c| c.id == contacto_id).unwrap();
        contactos.remove(index);

        contactos
    }
}
