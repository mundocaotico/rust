
use std::io;
use crate::gestion::gestionPersonas::gestionPersonas;
mod gestion;
mod mascota;
mod persona;

fn main() {
    let mut gestion_personas: gestionPersonas = gestionPersonas::new();
    loop {
        let mut opcion = 0;
        let mut dato = String::new();
        println!("\t----MENU----\t");
        println!("1: Crear personas");
        println!("2: Mostrar personas");
        println!("3: Buscar persona");
        println!("4: Salir");

        println!("Introduce una opcion: ");

        io::stdin().read_line(&mut dato).unwrap();

        opcion = match dato.trim().parse() {
            Ok(n) => n,
            Err(e) => {
                println!("Ha ocurrido un error: {}", e);
                continue;
            }
        };

        match opcion {
            1 => {
                gestion_personas.crearPersonas();
            },
            2 => {
                gestion_personas.mostrarPersonas();
            },
            3 => {
                gestion_personas.buscarPersona();
            },
            4 => {
                gestion_personas.salir();
            },
            _ => {
                println!("Valor fuera de rango");
                continue;
            }
        }

    }
}