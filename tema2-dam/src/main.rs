use std::ascii::AsciiExt;
use std::io;
use std::io::Read;
use std::num::ParseIntError;
use std::ops::Add;

fn main() {
    let mut mi_nombre: String = String::from("Ian");
    println!("Bienvenido, {}", mi_nombre);

    // Ejercicio 2: Pedir nombre
    mi_nombre = String::new();
    println!("Introduce tu nombre: ");
    io::stdin().read_line(&mut mi_nombre).unwrap();

    println!("Bienvenido, {}", mi_nombre);

    // Ejercicio 3: Calcular volumen de una esfera

    const PI: f64 = std::f64::consts::PI;
    let mut dato: String = String::new();
    let mut radio: f64;
    let mut volumen: f64;

    println!("Introduce el radio de la esfera: ");
    io::stdin().read_line(&mut dato).expect("Error en la entrada");

    radio = dato.trim().parse::<f64>().expect("La entrada no es un numero flotante");
    volumen = (4.0 / 3.0) * PI * radio.powi(3);

    println!("Volumen de la esfera: {} cm cubicos", volumen);
    dato = String::new();
    // Ejercicio 4: Calcular longitud de circunferencia

    println!("Introduce el radio de la circunferencia: ");
    io::stdin().read_line(&mut dato).expect("Error en la entrada");

    radio = dato.trim().parse::<f64>().expect("La entrada no es un numero flotante"); // unwrap() sirve, pero no lleva argumento
    volumen = 2_f64 * PI * radio; // La usamos para almacenar la longitud en este ejercicio

    println!("Longitud de la circunferencia: {}", volumen);

    dato = String::new();

    // Ejercicio 5: Leer caracter(es) y mostrar el entero ASCII

    let mut caracter: String = String::new();

    println!("Introduce el caracter: ");
    io::stdin().read_line(&mut caracter).unwrap();

    for c in caracter.trim().chars() {
        println!("El caracter {c} es igual a: {:?}", String::from(c).as_bytes());
    }

    //("Caracter {} es {:?}", caracter,caracter.as_bytes());

    // Ejercicio 6: Entero a caracter

    /*
    caracter = String::new();
    println!("Introduce el codigo ascii del caracter: ");
    io::stdin().read_line(&mut caracter).unwrap();
    println!("{:?}",caracter as u8);
    */

    // Ejercicio 7: Pasar grados centigrados a fahrenheit
    let mut celsius = String::new();
    println!("Introduce la temperatura en grados centigrados: ");
    io::stdin().read_line(&mut celsius).expect("Error al leer la entrada");
    let mut n = celsius.trim().parse::<f32>().expect("Error al convertir a f32");

    println!("Temperatura en grados fahrenheit: {}", 32.0 + (9_f32 * n) / 5.0);

    // Ejercicio 8: Convertir km/h a m/s

    let mut kmPorHora: String = String::new();
    let mut metrosPorSegundo: f32 = 0.0;

    println!("Introduce los kilometros por hora: ");
    io::stdin().read_line(&mut kmPorHora).expect("Error al leer la entrada");

    metrosPorSegundo = match kmPorHora.trim().parse::<f32>() {
        Ok(n) => (n * 1000.0) / 3600.0,
        Err(_) => {
            println!("Ha ocurrido un error");
            return;
        }
    };

    println!("Metros por segundo: {}", metrosPorSegundo);

    // Ejercicio 9: Calcular hipotenusa segun teorema de pitagoras

    let mut cateto1: String = String::new();
    let mut cateto2: String = String::new();
    let mut hipotenusa: f32 = 0_f32;
    println!("Introduce la longitud del cateto 1: ");
    io::stdin().read_line(&mut cateto1).expect("Error al leer la entrada");
    println!("Introduce la longitud del cateto 2: ");
    io::stdin().read_line(&mut cateto2).expect("Error al leer la entrada");

    hipotenusa = (cateto1.trim().parse::<f32>().unwrap().powi(2) + cateto2.trim().parse::<f32>().unwrap().powi(2)).sqrt();

    println!("La hipotenusa es de: {} cm", hipotenusa);

    // Operadores lógicos y relacionales

    // Ejercicio 10: Comprobar mayor de edad
    let mut entrada = String::new();
    let mut edad: Result<i32, ParseIntError>;
    let mut mayorDeEdad: bool = false;

    //io::stdin().read_exact(&mut edad).unwrap();

    println!("Introduce tu edad: ");
    io::stdin().read_line(&mut entrada).expect("Error al leer la entrada");

    edad = match entrada.trim().parse::<i32>() {
        Ok(n) => Ok(n),
        Err(_) => {
            println!("Error. El dato no es un numero entero");
            return;
        }
    };

    if (edad.unwrap() >= 18) {
        mayorDeEdad = true;
        println!("Mayor de edad: {}", mayorDeEdad);
    } else {
        println!("Mayor de edad: {}", mayorDeEdad);
    }


}
