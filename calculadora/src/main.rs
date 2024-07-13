
use std::io;

fn main() {

    let mut _opcion: String;

    let mut _numero1: f32;
    let mut _numero2: f32;

    loop {
        _opcion = String::new();
        let mut num1: String = String::new();
        let mut num2: String = String::new();

        println!("Menu");
        println!("1: Sumar");
        println!("2: Restar");
        println!("3: Multiplicar");
        println!("4: Dividir");
        println!("5: Salir");

        println!("Introduce una opcion: ");

    
        io::stdin().read_line(&mut _opcion).expect("No se ha podido leer la linea");

        match _opcion.trim().parse() {
            Ok(1) => {
                println!("Has seleccionado la opcion 1");

                println!("Introduce un numero: ");
                io::stdin().read_line(&mut num1).expect("Error al leer el numero");
                println!("Introduce el segundo numero: ");
                io::stdin().read_line(&mut num2).expect("Error al leer el numero");

                //Convertimos los string a numeros
                _numero1 = match num1.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Entrada no valida para el numero");
                        break;
                    }
                };
                _numero2 = match num2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Entrada no valida para el numero");
                        break;
                    }
                };

                //Hacemos la operacion

                println!("{}",sumar(_numero1, _numero2));

            },
            Ok(2) => {
                println!("Has seleccionado la opcion 2");

                println!("Introduce un numero: ");
                io::stdin().read_line(&mut num1).expect("Error al leer el numero");
                println!("Introduce el segundo numero: ");
                io::stdin().read_line(&mut num2).expect("Error al leer el numero");
                //resultado = restar(_numero1, _numero2);

                //Convertimos los string a numeros
                _numero1 = match num1.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Entrada no valida para el numero");
                        break;
                    }
                };

                _numero2 = match num2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Entrada no valida para el numero");
                        break;
                    }
                };

                //Realizamos la operacion

                println!("{}",restar(_numero1,_numero2));

            },
            Ok(3) => {
                println!("Has seleccionado la opcion 3");

                println!("Introduce un numero: ");
                io::stdin().read_line(&mut num1).expect("Error al leer el numero");
                println!("Introduce el segundo numero: ");
                io::stdin().read_line(&mut num2).expect("Error al leer el numero");

                //Convertimos los string a numeros

                _numero1 = match num1.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Entrada no valida para el numero");
                        break;
                    } 
                };

                _numero2 = match num2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Entrada no valida para el numero");
                        break;
                    }
                };

                //Realizamos la operacion

                println!("{}", multiplicar(_numero1, _numero2));

            },
            Ok(4) => {
                println!("Has seleccionado la opcion 4");

                println!("Introduce un numero: ");
                io::stdin().read_line(&mut num1).expect("Error al leer el numero");
                println!("Introduce el segundo numero: ");
                io::stdin().read_line(&mut num2).expect("Error al leer el numero");

                //Convertimos los string a numeros

                _numero1 = match num1.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Entrada no valida para el numero");
                        break;
                    }
                };

                _numero2 = match num2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Entrada no valida para el numero");
                        break;
                    }
                };

                println!("{}",dividir(_numero1, _numero2));

            },
            Ok(5) => {
                println!("Saliendo...");
                break;
            },
            _ => {
                println!("Opcion no valida, intente nuevamente");
            }
        }

    }

/*
    _resultado = sumar(numero1, numero2) as f32;
    _resultado = restar(numero1, numero2) as f32;
    _resultado = multiplicar(numero1 as f32,numero2 as f32);
    _resultado = dividir(numero1 as f32, numero2 as f32);
    //println!("{}",resultado);
    */

}

fn sumar(num1: f32, num2: f32) -> f32 {
    num1 + num2
}

fn restar(num1: f32, num2: f32) -> f32 {
    num1 - num2
}

fn multiplicar(num1: f32, num2: f32) -> f32 {
    num1 * num2
}

fn dividir(num1: f32, num2: f32) -> f32 {
    if (num2 != 0_f32) {
        num1 / num2
    } else {
        0_f32 // o 0 as f32
    }
    
}