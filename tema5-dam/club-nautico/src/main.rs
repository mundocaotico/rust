use std::fmt;
use std::fmt::{Display, Formatter, Pointer};
use std::io::stdin;
use std::process::exit;
#[derive(Debug, Clone)]
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
    fn get_eslora(&self) -> f32 {
        self.eslora
    }
    fn get_matricula(&self) -> &str {
        self.matricula.as_str()
    }
    fn set_eslora(&mut self, eslora: f32) {
        self.eslora = eslora;
    }
    fn set_matricula(&mut self, matricula: String) {
        self.matricula = matricula;
    }
    fn new () -> Self {
        Barco {
            eslora: 0.0,
            matricula: String::new()
        }
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
    fn get_nombre(&self) -> &str {
        self.nombre.as_str()
    }
    fn get_dni(&self) -> &str {
        self.dni.as_str()
    }
    fn get_barco(&self) -> &Option<Barco> {
        &self.barco
    }
    fn set_nombre(&mut self, nombre: String) {
        self.nombre = nombre;
    }
    fn set_dni(&mut self, dni: String) {
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
        write!(f,"{}\n{}\n{:?}", self.get_nombre(), self.get_dni(), self.get_barco())
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
                        usuario.set_dni(String::from(x));
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
                        usuario.set_nombre(String::from(x));
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
                let mut dato = String::new();

                println!("Que parametro quiere usar para buscar al usuario? (dni/matricula): ");
                stdin().read_line(&mut dato).expect("Error al leer la entrada");

                let input = dato.trim().to_lowercase();

                match input.as_str() {
                    "dni" => {
                        dni.clear();
                        println!("Introduce el dni del usuario: ");
                        stdin().read_line(&mut dni).expect("Error al leer la entrada");

                        let dni_input = dni.trim();

                        for u in usuarios.iter() {
                            if u.get_dni().trim() == dni_input {
                                println!("Nombre: {} DNI: {} Barco: {:?}", u.get_nombre(), u.get_dni(), u.get_barco());
                            }
                        }
                    },
                    "matricula" => {
                        matricula.clear();
                        println!("Introduce la matricula del barco: ");
                        stdin().read_line(&mut matricula).expect("Error al leer la entrada");

                        let matricula_input = matricula.trim();

                        for u in usuarios.iter() {
                            if let Some(barco) = u.get_barco() {
                                if barco.get_matricula().trim() == matricula_input {
                                    println!("Nombre: {} DNI: {} Barco: {:?}", u.get_nombre(), u.get_dni(), u.get_barco());
                                }
                            }
                        }
                    },
                    _ => {
                        println!("Usted no ha introducido correctamente el parametro requerido");
                    }
                }
            },
            4 => {
                let mut dni = String::new();
                let mut barco = Barco::new();

                dato.clear();

                println!("Introduce el dni del usuario: ");
                stdin().read_line(&mut dato).expect("Error al leer la entrada");

                dni = match dato.trim() {
                    x => String::from(x),
                    _ => {
                        println!("Error con el dni");
                        return;
                    }
                };

                dato.clear();

                println!("Introduce los datos del barco");
                println!("Eslora: ");

                stdin().read_line(&mut dato).expect("Error al leer la entrada");

                match dato.trim().parse::<f32>() {
                    Ok(n) => &barco.set_eslora(n),
                    Err(_) => {
                        println!("Error al establecer el valor de eslora");
                        return;
                    }
                };

                dato.clear();

                println!("Matricula del barco: ");
                stdin().read_line(&mut dato).expect("Error al leer la entrada");

                match dato.trim() {
                    x => &barco.set_matricula(String::from(x)),
                    _ => {
                        println!("Error al establecer la matricula del barco");
                        return;
                    }
                };

                barcos.push(barco.clone());

                for u in usuarios.iter_mut() {
                    if u.get_dni() == dni {
                        let b = barco.clone();
                        u.setBarco(Option::from(b)); // -- value used after being moved
                    }
                }

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
                    if usuarios[i].dni.trim() == dato.trim() {
                        println!("Indice del usuario: {}",i);
                        usuarios.remove(i);
                    }
                }
            },

            7 => {
                //let nuevoBarco;
                let mut barco = Barco::new();
                let mut dni = String::new();

                dato.clear();

                println!("Introduce los datos del barco");
                println!("Eslora: ");

                stdin().read_line(&mut dato).expect("Error en la entrada");

                barco.set_eslora(match dato.trim().parse::<f32>() {
                    Ok(n) => {n},
                    Err(_) => {
                        println!("Formato de datos incorrecto");
                        return;
                    }
                });

                dato.clear();

                println!("Matricula: ");
                stdin().read_line(&mut dato).expect("Error al leer la entrada");

                barco.set_matricula( match dato.trim() {
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

                for u in usuarios.iter_mut() {
                        if u.get_dni() == dni {
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
