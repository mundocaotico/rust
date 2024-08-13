use std::fs::File;
use crate::persona::persona::Persona;
use crate::mascota::mascota::Mascota;
use std::io;
use std::io::BufWriter;


pub struct gestionPersonas {
    personas: Vec<Persona>
}

impl gestionPersonas {
    pub fn new() -> Self {
        Self {
            personas: Vec::new()
        }
    }
    pub fn crear_personas(&mut self) {
        let ruta = "personas.dat";
        let mut fichero: Option<File> = None;
        let mut nombre = String::new();
        let mut nombre_mascota = String::new();
        let mut tipo_mascota = String::new();
        let mut apellidos = String::new();
        let mut edad: u8 = 0;
        let mut dato = String::new();
        let n_personas: u32;

        println!("Cuantas personas quiere crear? ");

        io::stdin().read_line(&mut dato).unwrap();

        n_personas = match dato.trim().parse::<u32>() {
            Ok(n) => n,
            Err(e) => {
                println!("Error de conversion de datos: {}", e);
                return;
            }
        };

        dato.clear();
        for i in 1..=n_personas {
            let mut dato = String::new();
            println!("Introduce los datos de la persona");

            println!("Nombre: ");

            io::stdin().read_line(&mut nombre).unwrap();

            println!("Apellidos: ");

            io::stdin().read_line(&mut apellidos).unwrap();

            println!("Edad: ");

            io::stdin().read_line(&mut dato).expect("Error al leer la entrada");

            edad = match dato.trim().parse() {
                Ok(n) => if n > 0 {
                    n
                } else {
                    println!("La edad debe ser mayor a 0");
                    return;
                },
                Err(e) => {
                    println!("Error al parsear la edad: {}", e);
                    return;
                }
            };

            println!("Mascota: ");

            println!("Introduce el nombre de la mascota: ");

            io::stdin().read_line(&mut nombre_mascota).unwrap();

            println!("Introduce el tipo de mascota: ");

            io::stdin().read_line(&mut tipo_mascota).unwrap();

            // Introducimos los datos

            self.personas.push(Persona::new(
                String::from(nombre.trim()),
                String::from(apellidos.trim()),
                edad,
                Mascota::new(String::from(nombre_mascota.trim()), String::from(tipo_mascota.trim()))));

            nombre.clear();
            apellidos.clear();
            edad = 0;
            nombre_mascota.clear();
            tipo_mascota.clear();

        }

        // Parte de ficheros

        fichero = match File::open(ruta) {
            Ok(f) => Some(f),
            Err(_) => {
                Some(File::create(ruta).unwrap())
            }
        };

        if let Some(f) = fichero {
            let writer = BufWriter::new(f);
            serde_json::to_writer_pretty(writer, &self.personas).unwrap()
        }

    }
    pub fn mostrar_personas(&self) {
        let personas = &self.personas;
        for p in personas.iter() {
            println!("{}",p);
        }

    }
    pub fn buscar_persona(&self) {
        let mut nombre = String::new();
        let mut apellidos = String::new();
        println!("Introduce los siguientes datos (nombre y apellidos)");

        println!("Nombre: ");
        io::stdin().read_line(&mut nombre).unwrap();

        println!("Apellidos: ");
        io::stdin().read_line(&mut apellidos).unwrap();

        // Iterar entre personas

        for p in self.personas.iter() {
            if p.clone().get_nombre() == nombre && p.clone().get_apellidos() == apellidos {
                println!("{}",p);
            } else {
                println!("No se ha encontrado una persona con los datos proporcionados.");
            }
        }

    }
    pub fn salir(&self) {
        std::process::exit(0);
    }
}