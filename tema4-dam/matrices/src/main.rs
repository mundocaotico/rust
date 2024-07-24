use rand::prelude::*;
use std::io::stdin;
use std::process::exit;

fn main() {

    let mut matrix: [[i32; 10]; 10] = [[0; 10]; 10];
    let mut rng = thread_rng();
    for i in 0..matrix.iter().len() {
        for j in 0..matrix.iter().len() {
            matrix[i][j] = rng.gen_range(0..=20);
        }
    }

    for i in 0..matrix.iter().len() {
        for j in 0..matrix.iter().len() {
            println!("Posicion [{}][{}]: {}", i,j,matrix[i][j]);
        }
    }

    loop {
        let mut dato = String::new();
        let mut opcion = 0;
        println!("\t---MENU----\t");
        println!("1: Imprimir la suma de los numeros almacenados en la matriz");
        println!("2: Decir la posicion en la que se almacena el numero mayor");
        println!("3: Calcular la suma de cada fila y almacenar el resultado en un vector");
        println!("4: Calcular la suma de las columnas e indicar el numero de la columna con la suma mas alta y el valor de la suma");
        println!("5: Indicar cuantos numeros son mayores a 10 y cuantos menores o iguales");
        println!("6: Salir");

        println!("Introduce una opcion: ");
        stdin().read_line(&mut dato).expect("Error al leer la entrada");

        opcion = match dato.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("El valor no es un numero entero");
                return;
            }
        };

        match opcion {
            1 => {
                let mut suma = 0;
                println!("Imprimiendo suma de los numeros de la matriz");

                for i in 0..matrix.iter().len() {
                    for j in 0..matrix.iter().len() {
                        suma += matrix[i][j];
                    }
                }

                println!("Suma: {}", suma);

            },
            2 => {
                let mut mayor = -1;
                let mut posiciones = [0; 2];
                println!("Imprimiendo la posicion del numero mayor");

                for i in 0..matrix.iter().len() {
                    for j in 0..matrix.iter().len() {
                        if (mayor < matrix[i][j]) {
                            mayor = matrix[i][j];
                            posiciones[0] = i;
                            posiciones[1] = j;
                        }
                    }
                }
                println!("La posicion del numero mayor en la matriz es: Fila: {}, Columna: {}", posiciones[0],posiciones[1]);
            },
            3 => {
                let mut suma = 0;
                let mut vector_suma = [0; 10];
                println!("Mostrando suma de cada fila en un vector");

                for fila in 0..matrix.iter().len() {
                    for columna in 0..matrix.iter().len() {
                        suma += matrix[fila][columna];
                    }
                    vector_suma[fila] = suma;
                    suma = 0;
                }

                println!("Imprimiendo suma por fila");

                for fila in 0..vector_suma.iter().len() {
                    println!("Fila {}: {}", fila+1, vector_suma[fila]);
                }

            },
            4 => {
                let mut suma = 0;
                let mut posicion = 0;
                let mut suma_columna = [0; 10];
                println!("Mostrando suma de cada columna y columna con la suma mas alta y su valor");

                for i in 0..matrix.iter().len() {
                    for j in 0..matrix.iter().len() {
                        suma += matrix[j][i];
                    }
                    suma_columna[i] = suma;
                    suma = 0;
                }

                println!("Mostrando suma por columna");

                for i in 0..suma_columna.iter().len() {
                    println!("Columna {}: {}", i+1, suma_columna[i]);
                }

                println!("Mostrando columna con suma mas alta: ");

                for i in 0..suma_columna.iter().len() {
                    if suma < suma_columna[i] {
                        suma = suma_columna[i];
                        posicion = i;
                    }
                }

                println!("La columna con la suma mas alta es: {} con valor: {}", posicion+1, suma_columna[posicion]);

            },
            5 => {
                let mut contador = 0;
                println!("Mostrando cantidad de numeros mayores a 10 y cuantos menores o iguales");

                for i in 0..matrix.iter().len() {
                    for j in 0..matrix.iter().len() {
                        if (matrix[i][j] > 10) {
                            contador += 1;
                        }
                    }
                }

                println!("Hay {} numeros mayores a 10, y {} menores o iguales", contador, 100 - contador);

            },
            6 => {
                println!("Saliendo del programa");
                exit(0);
            },
            _ => {
                println!("Valor fuera del rango");
            }
        }

    }

}
