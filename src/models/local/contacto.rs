use std::{error::Error, fs, io::ErrorKind, path};

use crate::contacto::Contacto;

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

        let contacto = Contacto::nuevo(nombre, apellido, telefono, email);
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
