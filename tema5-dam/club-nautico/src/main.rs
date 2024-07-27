use std::fmt;
use std::fmt::{Display, Formatter, Pointer};
use std::io::stdin;
use std::process::exit;
#[derive(Debug)]
struct Barco {
    eslora: f32,
    matricula: String
}

struct Usuario {
    nombre: String,
    dni: String,
    barco: Option<Barco>
}

impl Barco {
    fn getEslora(&self) -> f32 {
        self.eslora
    }
    fn getMatricula(&self) -> &str {
        self.matricula.as_str()
    }
    fn setEslora(&mut self, eslora: f32) {
        self.eslora = eslora;
    }
    fn setMatricula(&mut self, matricula: String) {
        self.matricula = matricula;
    }
    fn new (eslora: f32, matricula: String) -> Barco {
        Barco {eslora, matricula}
    }

}
/*
impl Iterator for Barco {
    type Item = Barco;

    fn next(&mut self) -> Option<Self::Item> {
        Self::Item
    }
}
*/
impl Usuario {
    fn getNombre(&self) -> &str {
        self.nombre.as_str()
    }
    fn getDNI(&self) -> &str {
        self.dni.as_str()
    }
    fn getBarco(&self) -> &Option<Barco> {
        &self.barco
    }
    fn setNombre(&mut self, nombre: String) {
        self.nombre = nombre;
    }
    fn setDNI(&mut self, dni: String) {
        self.dni = dni;
    }
    fn setBarco(&mut self, barco: Option<Barco>) {
        self.barco = Option::from(barco);
    }
    pub fn new(nombre: String, dni: String, barco: Option<Barco>) -> Usuario {
        Usuario {nombre, dni, barco }
    }
}
/*
impl Iterator for Usuario {
    type Item = Usuario;

    fn next(&mut self) -> Option<Self::Item> {
        Self::Item
    }
}
*/
impl Display for Usuario {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}{}{:?}", self.getNombre(), self.getDNI(), self.getBarco())
    }
}

fn main() {
    let mut dato: String = String::with_capacity(20);
    let mut usuarios: Vec<Usuario> = Vec::with_capacity(10);
    let mut barcos: Vec<Barco> = Vec::with_capacity(10);
    let mut opcion = 0;

    loop {
        dato.clear();
        println!("\t---MENU----\t");
        println!("1: Insertar un nuevo usuario");
        println!("2: Mostrar todos los usuarios");
        println!("3: Buscar un usuario por dni o por matricula del barco");
        println!("4: Cambiar barco, Busca el usuario por DNI");
        println!("5: Cambiar eslora. Buscar barco por matricula");
        println!("6: Eliminar un usuario por DNI");
        println!("7: Insertar barco a un usuario");
        println!("8: Salir");

        println!();
        println!("Introduce una opcion");

        stdin().read_line(&mut dato).expect("Error al leer la entrada");

        opcion = match dato.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                println!("El valor debe ser un numero entero");
                return;
            }
        };

        match opcion {
            1 => {
                let mut usuario = Usuario::new(String::new(), String::new(), None);
                dato.clear();

                println!("Introduce el dni del usuario: ");
                stdin().read_line(&mut dato).expect("Error al leer la entrada");

                match &dato {
                    x => {
                        usuario.setDNI(String::from(x));
                    },
                    _ => {
                        println!("Error en los datos");
                        return;
                    }
                }

                dato.clear();

                println!("Introduce el nombre del usuario: ");
                stdin().read_line(&mut dato).expect("Error al leer la entrada");

                match &dato {
                    x => {
                        usuario.setNombre(String::from(x));
                    },
                    _ => {
                        println!("Error en los datos");
                        return;
                    }
                }

                usuarios.push(usuario);

            },
            2 => {

                println!("Mostrando lista de usuarios: ");

                if (usuarios.len() > 0) {
                    for u in usuarios.iter() {
                        println!("Usuario: {}", u.dni);
                    }
            } else {
                    println!("La lista de usuarios esta vacia");
                }
            },
            3 => {
                let mut dni: String = String::new();
                let mut matricula: String = String::new();
                dato.clear();

                println!("Que parametro quiere usar para buscar al usuario? (dni/matricula): ");

                stdin().read_line(&mut dato).expect("Error al leer la entrada");

                match &dato {
                    x => {
                        if String::from(x).trim().to_lowercase().eq("dni") {

                            dato.clear();
                            println!("Introduce el dni del usuario: ");

                            stdin().read_line(&mut dato).expect("Error al leer la entrada");

                            for mut u in usuarios.iter() {
                                    if u.getDNI() == dato {
                                        println!("{}", u);
                                    }
                            }

                        } else if String::from(x).trim().to_lowercase().eq("matricula") {

                            dato.clear();
                            println!("Introduce la matricula del barco: ");

                            stdin().read_line(&mut dato).expect("Error al leer la entrada");

                            for mut u in usuarios.iter() {
                                // TODO: toString() de usuario
                            }

                        } else {
                            println!("Usted no ha introducido correctamente el parametro requerido");
                        }
                    }
                }

            },
            4 => {
                let mut dni = String::new();



                dato.clear();

            },
            5 => {
                let mut matricula = String::new();

                dato.clear();

            },
            6 => {
                let mut contador = 0;
                let mut dni = String::new();
                dato.clear();
                println!("Introduce el dni del usuario: ");

                stdin().read_line(&mut dato).expect("Error al leer la entrada");
                dni = match &dato {
                    x => String::from(x),
                    _ => {
                        println!("Error en los datos");
                        return;
                    }
                };

                for i in 0..usuarios.iter().len() {
                    if usuarios.get(i).unwrap().dni == dato.trim() {
                        usuarios.remove(i);
                    }
                }
            },
                /*
                for u in usuarios.iter_mut() {
                        if u.getDNI() == dato.trim() {
                                usuarios.remove(contador);
                            } else {
                                println!("Error en el contador: {}", contador);
                            }

                        }

                    contador += 1;
                },
                 */
            7 => {
                //let nuevoBarco;
                let mut barco = Barco::new(0.0, String::new());
                let mut dni = String::new();

                dato.clear();

                println!("Introduce los datos del barco");
                println!("Eslora: ");

                barco.setEslora(match dato.trim().parse::<f32>() {
                    Ok(n) => {n},
                    Err(_) => {
                        println!("Formato de datos incorrecto");
                        return;
                    }
                });

                dato.clear();

                println!("Matricula: ");
                stdin().read_line(&mut dato).expect("Error al leer la entrada");

                barco.setMatricula( match dato.trim() {
                    s => String::from(s),
                    _ => {
                        println!("Error con el tipo de datos");
                        return;
                    }
                });

                //nuevoBarco = &barco;

                barcos.push(barco);

                dato.clear();

                println!("Introduce el dni del usuario al que añadir el barco: ");
                stdin().read_line(&mut dato).expect("Error al leer los datos");

                dni = match dato.trim() {
                    s => String::from(s),
                    _ => {
                        println!("Error con el tipo de dato");
                        return;
                    }
                };

                for mut u in usuarios.iter_mut() {
                        if u.getDNI() == dni {
                            //u.setBarco();
                        }
                }

            },
            8 => {
                exit(0);
            },
            _ => {
                println!("Valor fuera de rango");
            }
        }

    }

}
