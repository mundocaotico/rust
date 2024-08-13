use std::fmt;
use std::fmt::{Display, Formatter};
use crate::mascota::mascota::Mascota;
use serde::{Deserialize, Serialize};


#[derive(Clone,Serialize, Deserialize)]
pub(crate) struct Persona {
    nombre: String,
    apellidos: String,
    edad: u8,
    mascota: Mascota
}

impl Persona {

    pub fn new(nombre: String, apellidos: String, edad: u8, mascota: Mascota) -> Self {
        Persona {
            nombre,
            apellidos,
            edad,
            mascota
        }
    }
    pub fn set_nombre(&mut self,nombre: String) {
        self.nombre = nombre;
    }
    pub fn set_apellidos(&mut self,apellidos: String) {
        self.apellidos = apellidos;
    }
    pub fn set_edad(&mut self,edad: u8) {
        self.edad = edad;
    }
    pub fn set_mascota(&mut self,mascota: Mascota) {
        self.mascota = mascota;
    }
    pub fn get_nombre(self) -> String {
        self.nombre
    }
    pub fn get_apellidos(self) -> String {
        self.apellidos
    }
    pub fn get_edad(self) -> u8 {
        self.edad
    }
    pub fn get_mascota(self) -> Mascota {
        self.mascota
    }

}
impl Display for Persona {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"Nombre: {}\tApellidos: {}\tEdad: {}\nMascota: {}\n",
               self.nombre, self.apellidos, self.edad, self.mascota)
    }
}