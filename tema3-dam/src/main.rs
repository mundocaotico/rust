
use std::io::stdin;

fn main() {

    // Ejercicio 1: Indicar si dos numeros son iguales
    let mut entrada: String = String::new();
    let mut num1: i32 = 0;
    let mut num2: i32 = 0;

    println!("Introduce el primer numero: ");
    stdin().read_line(&mut entrada).expect("Error al leer la entrada");

    num1 = match entrada.trim().parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            println!("Error: El valor no es un numero entero");
            return;
        }
    };

    entrada = String::new();

    println!("Introduce el segundo numero: ");
    stdin().read_line(&mut entrada).expect("Error al leer la entrada");

    num2 = match entrada.trim().parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            println!("El valor no es un numero entero");
            return;
        }
    };

    if num1 == num2 {
        println!("Ambos valores son iguales");
    } else {
        println!("Los valores son diferentes: {} y {}", num1, num2);
    }

    // Ejercicio 2: Indicar si un numero es positivo, negativo, o cero

    entrada.clear();

    println!("Introduce el numero entero a comprobar: ");
    stdin().read_line(&mut entrada).expect("Error al leer la entrada");

    num1 = match entrada.trim().parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            println!("El valor no es un numero entero");
            return;
        }
    };

    if num1 > 0 {
        println!("El numero es positivo");
    } else if num1 < 0 {
        println!("El numero es negativo");
    } else {
        println!("El numero es igual a 0");
    }

    // Ejercicio 3: Leer dos numeros enteros por teclado y un operador

    entrada.clear();
    let mut resultado = 0;
    let mut operador: char;

    println!("Introduce el primer numero: ");
    stdin().read_line(&mut entrada).expect("Error al leer la entrada");

    num1 = match entrada.trim().parse() { // Es posible usarlo sin ::<>, pues puede inferir el tipo
        Ok(n) => n,
        Err(_) => {
            println!("El valor no es un entero");
            return;
        }
    };

    entrada.clear();

    println!("Introduce el segundo numero: ");
    stdin().read_line(&mut entrada).expect("Error al leer la entrada");

    num2 = match entrada.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("El valor no es un entero");
            return;
        }
    };

    entrada.clear();

    println!("Introduce el operador a utilizar (+,-,*,/): ");

    stdin().read_line(&mut entrada).expect("Error al leer la entrada");

    operador = match entrada.chars().next() {
        Some(op) => op,
        None => {
            println!("No se introdujo un operador válido");
            return;
        }
    };

    resultado = match operador {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => {
            if num2 == 0 {
                println!("No se puede dividir por cero");
                return;
            }
            num1 / num2
        }
        _ => {
            println!("Operador no válido");
            return;
        }
    };

    println!("El resultado de {} {} {} es: {}", num1, operador, num2, resultado);

    // Ejercicio 4: Pedir numero del mes y mostrar su número de días
    let mut mes: u8;
    let mut dias: u8;
    entrada.clear();

    println!("Introduce el número del mes: ");
    stdin().read_line(&mut entrada).expect("Error al leer la entrada");

    mes = match entrada.trim().parse::<u8>() {
        Ok(n) => n,
        Err(_) => {
            println!("El valor no es un numero entero");
            return;
        }
    };

    if mes == 2 {
        dias = 28;
    } else if mes == 4 || mes  == 6 || mes == 9 || mes == 11 {
        dias = 30;
    } else {
        dias = 31;
    }

    match dias {
        28 => println!("El mes {} tiene {} dias",mes,dias),
        30 => println!("El mes {} tiene {} dias",mes,dias),
        31 => println!("El mes {} tiene {} dias",mes,dias),
        _ => println!("No hay ningun mes con esos dias")
    };

    // Ejercicio 5: Generar 10 numeros de 0 a 20 y calcular la suma

    use rand::prelude::*;
    let mut rng = rand::thread_rng();
    let mut num: i32;
    let mut suma: i32 = 0;

    for n in 1..=10 {
        num = rng.gen_range(0..=20);
        suma += num;
    }

    println!("La suma de los numeros es: {}", suma);

    // Ejercicio 6: Pedir un numero y calcular su factorial

    entrada.clear();

    println!("Introduce un numero entero: ");
    stdin().read_line(&mut entrada).expect("Error al leer la entrada");

    num = match entrada.trim().parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            println!("El valor no es un numero entero");
            return;
        }
    };

    resultado = 1;

    for i in 1..=num {
        resultado *= i;
    }

    println!("El factorial de {} es {}", num, resultado);

    // Ejercicios while

    // Ejercicio 7: Pedir un numero e indicar si es positivo o negativo. Se repite hasta que se introduzca 0

    num = 1;
    println!("Ejercicio while 1");
    while (num != 0) {
        entrada.clear();
        println!("Introduce un numero entero: ");
        stdin().read_line(&mut entrada).expect("Error al leer la entrada");

        num = match entrada.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_)  => {
                println!("El valor no es un numero entero");
                return;
            }
        };

        if (num > 0) {
            println!("El numero {} es positivo", num);
        } else if (num < 0) {
            println!("El numero {} es negativo", num);
        } else {
            println!("El numero es 0");
        }

    }

    // Ejercicio 8: Pedir numeros hasta que se introduzca uno negativo y calcular la media (P: acumuladores)

    num = 0;
    println!("Ejercicio while 2");
    let mut contador = 0;
    suma = 0;
    while (num >= 0) {
        entrada.clear();
        println!("Introduce un numero entero: ");
        stdin().read_line(&mut entrada).expect("Error al leer la entrada");

        num = match entrada.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                println!("El valor no es un entero");
                return;
            }
        };
        contador += 1;
        suma += num;

    }

    println!("La media de los {} numeros es: {}", contador, (suma / contador) as f32);

    // Ejercicio completo: Actividades integradas

    // Ejercicio 9: Pedir 3 numeros y hacer un menu con 4 opciones (mostrar menor, son impares, pedir numeros, y salir)
    entrada.clear();
    num = 0;
    num1 = 0;
    num2 = 0;

    println!("Introduce un entero: ");
    stdin().read_line(&mut entrada).expect("Error al leer la entrada");

    num = match entrada.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("El valor no es un entero");
            return;
        }
    };

    entrada.clear();

    println!("Introduce otro entero: ");
    stdin().read_line(&mut entrada).expect("Error al leer la entrada");

    num1 = match entrada.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("El valor no es un entero");
            return;
        }
    };

    entrada.clear();

    println!("Introduce otro entero mas: ");
    stdin().read_line(&mut entrada).expect("Error al leer la entrada");

    num2 = match entrada.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("El valor no es un entero");
            return;
        }
    };

    entrada.clear();

    loop {
        let mut opcion = 0;

        println!("Menu");
        println!("1: Mostrar el menor");
        println!("2: Son impares?");
        println!("3: Pedir numeros");
        println!("4: Salir");

        entrada.clear();
        println!("Introduce una opcion: ");
        stdin().read_line(&mut entrada).expect("Error al leer la entrada");

        opcion = match entrada.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                println!("El valor no es entero");
                return;
            }
        };

        match opcion {
            1 => {
                println!("Mostrando el menor");
                if (num < num1 && num < num2) {
                    println!("El numero menor es {}", num);
                } else if (num1 < num && num < num2) {
                    println!("El numero menor es {}", num1);
                } else {
                    println!("El numero menor es {}", num2);
                }
            },
            2 => {
                println!("Mostrando si los numeros son impares");
                if (num % 2 != 0 && num1 % 2 != 0 && num2 != 0) {
                    println!("Todos los numeros son impares");
                } else {
                    println!("No todos los numeros son impares");
                }
            },
            3 => {
                println!("Volviendo a pedir numeros");

                entrada.clear();
                num = 0;
                num1 = 0;
                num2 = 0;

                println!("Introduce un entero: ");
                stdin().read_line(&mut entrada).expect("Error al leer la entrada");

                num = match entrada.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("El valor no es un entero");
                        return;
                    }
                };

                entrada.clear();

                println!("Introduce otro entero: ");
                stdin().read_line(&mut entrada).expect("Error al leer la entrada");

                num1 = match entrada.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("El valor no es un entero");
                        return;
                    }
                };

                entrada.clear();

                println!("Introduce otro entero mas: ");
                stdin().read_line(&mut entrada).expect("Error al leer la entrada");

                num2 = match entrada.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("El valor no es un entero");
                        return;
                    }
                };

                entrada.clear();

            },
            _ => {
                println!("Saliendo...");
                std::process::exit(0);
            }
        }

    }

}
