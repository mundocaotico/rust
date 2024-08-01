use std::io::{IsTerminal, Read};
use std::num::ParseIntError;
use crate::nota::Nota;

mod nota;

fn main() {
    let mut nota: Nota = unsafe { Nota::new() };
    let mut notas: Vec<Nota> = Vec::new();
    nota.set_prioridad(1);
    nota.set_texto(String::from("Texto de prueba"));

    println!("{}", nota);

    loop {
        let mut dato: String = String::new();
        let mut opcion: u8 = 0;

        println!("\t----MENU----\t");
        println!("1. Agregar nota");
        println!("2. Mostrar notas");
        println!("3. Modificar prioridad y/o texto de nota");
        println!("4. Borrar nota");
        println!("5. Salir");

        println!();
        println!("Introduce una opcion: ");

        std::io::stdin().read_line(&mut dato).expect("Error al leer la entrada");

        opcion = match dato.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Error de conversion de datos");
                return;
            }
        };

        match opcion {
            1 => unsafe {
                let mut nota: Nota = Nota::new();
                dato.clear();
                println!("Introduce el texto de la nota: ");
                std::io::stdin().read_line(&mut dato).expect("Error al leer la entrada");

                nota.set_texto(dato);

                nota.add_nota(&mut notas);

            },
            2 => {

                println!("Mostrando notas");

                for n in notas.iter() {
                    println!("{}",n);
                }

            },
            3 => {
                dato.clear();
                let mut eleccion = 0;
                println!("Modificar texto o prioridad?");
                println!("1: Texto");
                println!("2: Prioridad");
                println!("3: Ambos");

                println!();
                println!("Introduce un valor: ");

                std::io::stdin().read_line(&mut dato).expect("Error al leer la entrada");

                match dato.trim().parse::<u8>() {
                    Ok(n) => eleccion = n,
                    Err(_) => {
                        println!("Error de conversion de datos");
                        continue; // Añadir `continue` para no salir del loop en caso de error
                    }
                };

                match eleccion {
                    1 => {
                        let mut id_nota: u32;
                        dato.clear();
                        println!("Introduce el id de la nota a cambiar: ");
                        std::io::stdin().read_line(&mut dato).expect("Error al leer la entrada");

                        id_nota = match dato.trim().parse::<u32>() {
                            Ok(n) => n,
                            Err(_) => {
                                println!("Error de tipo de dato");
                                continue;
                            }
                        };

                        dato.clear();
                        println!("Introduce el nuevo texto de la nota: ");
                        std::io::stdin().read_line(&mut dato).expect("Error al leer la entrada");
                        let nuevo_texto = dato.trim().to_string();

                        let mut nota_encontrada = false;
                        for n in notas.iter_mut() {
                            if id_nota == n.get_id() {
                                n.set_texto(nuevo_texto);
                                nota_encontrada = true;
                                break;
                            }
                        }

                        if !nota_encontrada {
                            println!("No se encontró una nota con el id especificado.");
                        } else {
                            println!("Texto de la nota con id {} cambiado.", id_nota);
                        }

                    },
                    2 => {
                        let mut prioridad: i8;
                        let mut id_nota: u32;
                        dato.clear();
                        println!("Introduce el id de la nota a cambiar: ");
                        std::io::stdin().read_line(&mut dato).expect("Error al leer la entrada");

                        id_nota = match dato.trim().parse::<u32>() {
                            Ok(n) => n,
                            Err(_) => {
                                println!("Error de tipo de dato");
                                continue;
                            }
                        };

                        dato.clear();
                        println!("Introduce la nueva prioridad de la nota (-1 o 1): ");
                        std::io::stdin().read_line(&mut dato).expect("Error al leer la entrada");

                        match dato.trim().parse::<i8>() {
                            Ok(n) => {
                                if n == 1 || n == -1 {
                                    prioridad = n;
                                    let mut nota_encontrada = false;
                                    for n in notas.iter_mut() {
                                        if id_nota == n.get_id() {
                                            n.set_prioridad(prioridad);
                                            nota_encontrada = true;
                                            break;
                                        }
                                    }

                                    if !nota_encontrada {
                                        println!("No se encontró una nota con el id especificado.");
                                    } else {
                                        println!("Prioridad de la nota con id {} cambiada.", id_nota);
                                    }
                                } else {
                                    println!("Por favor introduzca un valor válido (-1 o 1)");
                                }
                            },
                            Err(_) => {
                                println!("Error en el tipo de dato");
                                continue;
                            }
                        }
                    },
                    3 => {
                        let mut prioridad: i8;
                        let mut id_nota: u32;
                        dato.clear();
                        println!("Introduce el id de la nota a cambiar: ");
                        std::io::stdin().read_line(&mut dato).expect("Error al leer la entrada");

                        id_nota = match dato.trim().parse::<u32>() {
                            Ok(n) => n,
                            Err(_) => {
                                println!("Error de tipo de dato");
                                continue;
                            }
                        };

                        dato.clear();
                        println!("Introduce la prioridad de la nota (-1 o 1): ");
                        std::io::stdin().read_line(&mut dato).expect("Error al leer la entrada");

                        match dato.trim().parse::<i8>() {
                            Ok(n) => {
                                if n == 1 || n == -1 {
                                    prioridad = n;
                                    dato.clear();

                                    println!("Introduzca el texto de la nota: ");
                                    std::io::stdin().read_line(&mut dato).expect("Error al leer la entrada");
                                    let nuevo_texto = dato.trim().to_string();

                                    let mut nota_encontrada = false;
                                    for n in notas.iter_mut() {
                                        if id_nota == n.get_id() {
                                            n.set_prioridad(prioridad);
                                            n.set_texto(nuevo_texto);
                                            nota_encontrada = true;
                                            break;
                                        }
                                    }

                                    if !nota_encontrada {
                                        println!("No se encontró una nota con el id especificado.");
                                    } else {
                                        println!("Prioridad y texto de la nota con id {} cambiados.", id_nota);
                                    }

                                } else {
                                    println!("Por favor introduzca un valor válido (-1 o 1)");
                                }
                            },
                            Err(_) => {
                                println!("Error en el tipo de dato");
                                continue;
                            }
                        }
                    },
                    _ => {
                        println!("Valor fuera de rango");
                    }
                }
            },
            4 => {
                let id: u32;
                dato.clear();
                println!("Introduce el id de la nota a borrar: ");

                std::io::stdin().read_line(&mut dato).expect("Error en la entrada");

                id = match dato.trim().parse::<u32>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Error de tipo de datos");
                        return;
                    }
                };

                for i in 0..notas.len() {
                    if notas[i].get_id() == id {
                        notas.remove(i);
                    }
                }

            },
            5 => {
                std::process::exit(0);
            },
            _ => {
                println!("Valor fuera de rango");
            }
        }

    }

}
