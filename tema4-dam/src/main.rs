
fn main() {

    // Ejercicio 1: Mostrar cantidad de vocales en un string
    let ejercicio1 = String::from("esternocleidomastoideo");
    let mut contador = 0;
    for i in ejercicio1.chars() {
        match i {
            'a' => contador += 1,
            'e' => contador += 1,
            'i' => contador += 1,
            'o' => contador += 1,
            'u' => contador += 1,
            _ => {}
        }
    }

    println!("La cantidad de vocales (puede haber repetidas) es: {}", contador);

    // Ejercicio 2: Invertir un string y mostrarlo por pantalla
    let ejercicio2 = String::from("casa blanca");
    let ejercicio2_reverse: String = ejercicio2.chars().rev().collect();

    println!("{}", ejercicio2_reverse);

    // Ejercicio 3: Verificar cuantas veces se repite un caracter en una cadena
    contador = 0;
    let ejercicio3 = String::from("Prueba de repeticion");
    let caracterEjercicio3: char = 'e';

    for i in ejercicio3.chars() {
        if (i == caracterEjercicio3) {
            contador += 1;
        }
    }

    println!("El caracter {} se repite {} veces", caracterEjercicio3, contador);

    // Ejercicio 6: Decir si un numero es capicua
    let ejercicio4: String = String::from("12321");
    let ejercicio4Reverse = ejercicio4.chars().rev().collect::<String>();

    if ejercicio4.eq(&ejercicio4Reverse) {
        println!("El numero es capicua");
    } else {
        println!("El numero no es capicua");
    }

    // Ejercicio 7: Encontrar palabra mas corta (comparar cadenas)

    let ejercicio5_0 = String::from("dinosaurio");
    let ejercicio5_1 = String::from("estratosfera");

    if ejercicio5_0.len() > ejercicio5_1.len() {
        println!("longitudes {} y {}", ejercicio5_0.len(), ejercicio5_1.len());
        println!("La palabra mayor es: {}", ejercicio5_0);
    } else {
        println!("La palabra mayor es: {}", ejercicio5_1);
    }

    // Ejercicio 8: Indicar cuantos espacios en blanco tiene una frase

    let mut ejercicio8 = String::new();
    let mut cantidadEspacios = 0;
    println!("Introduce una frase: ");
    std::io::stdin().read_line(&mut ejercicio8).unwrap();
    cantidadEspacios = ejercicio8.len() - ejercicio8.replace(" ","").len();
    println!("La frase tiene {} espacios", cantidadEspacios);


}
