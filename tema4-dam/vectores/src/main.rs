use std::process::exit;

fn main() {
    let mut dato: String = String::new();
    let mut edades: [u32; 10] = [0; 10];

    for i in 0..edades.len() {
        println!("Introduce una edad: ");
        std::io::stdin().read_line(&mut dato).unwrap();
        edades[i] = match dato.trim().parse::<u32>() {
            Ok(n) => n,
            Err(_) => {
                println!("El valor no es un numero entero");
                return;
            }
        };
        dato.clear();
        while edades[i] == 0 {
            dato.clear();
            println!("El valor es 0, introduce otro valor");
            std::io::stdin().read_line(&mut dato).unwrap();
            edades[i] = match dato.trim().parse::<u32>() {
                Ok(n) => n,
                Err(_) => {
                    println!("El valor no es un numero entero");
                    return;
                }
            };
        }
    }

    loop {
        dato.clear();
        let mut opcion = 0;

        println!("\t----MENU----\t");
        println!("1: Mostrar todas las edades introducidas");
        println!("2: Ordenar de menor a mayor las edades");
        println!("3: Buscar una edad en la lista y mostrar cuantas veces aparece");
        println!("4: Mostrar la edad media");
        println!("5: Mostrar el número de personas mayores de edad");
        println!("6: Mostrar la edad más alta");
        println!("7: Mostrar el número de edades superior a la media");
        println!("8: Salir");

        println!();
        println!("Introduce una opcion: ");

        std::io::stdin().read_line(&mut dato).expect("Error al leer la entrada");

        opcion = match dato.trim().parse() {
            Ok(n) => {
                if n <= 8 && n > 0 {
                    n
                } else {
                    println!("Por favor introduce un valor valido");
                    continue;
                }
            },
            Err(_) => {
                println!("Ha ocurrido un error");
                return;
            }
        };

        match opcion {
            1 => {
                println!("Mostrando edades introducidas: ");
                for i in 0..edades.iter().len() {
                    println!("{}",edades[i]);
                }
            },
            2 => {
                println!("Ordenando de menor a mayor las edades");
                let mut edadesOrdenadas = edades.clone();

                for i in 0..edadesOrdenadas.iter().len() {
                    for j in 0..edadesOrdenadas.iter().len() - 1 - i {
                        if edadesOrdenadas[j] > edadesOrdenadas[j+1] {
                            edadesOrdenadas.swap(j, j+1);
                        }
                    }
                }

                println!("Edades ordenadas");
                for i in 0..edadesOrdenadas.len() {
                    println!("{}",edadesOrdenadas[i]);
                }

            },
            3 => {
                println!("Introduce la edad a buscar en la lista");
                dato.clear();
                let mut contador = 0;
                let edad: u32;

                std::io::stdin().read_line(&mut dato).expect("Error al leer la entrada");

                edad = match dato.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("La edad debe ser un entero");
                        return;
                    }
                };

                for i in 0..edades.iter().len() {
                    if (edades[i] == edad) {
                        contador += 1;
                        println!("Edad encontrada en posicion {}",i+1);
                    }
                }

                println!("La edad {} se ha encontrado {} veces", edad, contador);

            },
            4 => {
                println!("Mostrando la media de edad");
                let mut suma: u32 = 0;
                let media: f32;
                for i in 0..edades.iter().len() {
                    suma += edades[i];
                }
                media = suma as f32 / edades.len() as f32;
                println!("La media de edad es {}", media);

            },
            5 => {
                println!("Mostrando cantidad de personas mayores de edad");
                let mut contador = 0;

                for i in 0..edades.iter().len() {
                    if edades[i] >= 18 {
                        contador += 1;
                    }
                }

                println!("La cantidad de personas mayores de edad es: {}", contador);

            },
            6 => {
                println!("Mostrando la edad mas alta");
                let mut mayor = 0;

                for i in 0..edades.iter().len() {
                    if edades[i] >= mayor {
                        mayor = edades[i];
                    }
                }

                println!("La edad mas alta es: {}", mayor);

            },
            7 => {
                println!("Mostrando cantidad de edades mayores a la media");
                let mut contador = 0;
                let mut suma: u32 = 0;
                let mut media: f32 = 0.0;
                for i in 0..edades.iter().len() {
                    suma += edades[i];
                }
                media = suma as f32 / edades.len() as f32;

                for i in 0..edades.iter().len() {
                    if (edades[i] > media as u32) {
                        contador += 1;
                    }
                }

                println!("Cantidad de personas mayores a la media: {}", contador);

            },
            8 => {
                println!("Saliendo del programa");
                exit(0);
            },
            _ => {
                println!("Valor no valido");
            }
        }

    }

}
