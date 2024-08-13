use std::fmt;
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Clone,Serialize, Deserialize)]
pub(crate) struct Mascota {
    nombre: String,
    tipoMascota: String
}

impl Mascota {

    pub fn new(nombre: String, tipoMascota: String) -> Self {
        Mascota {
            nombre,
            tipoMascota
        }
    }

    pub fn set_nombre(&mut self,nombre: String) {
        self.nombre = nombre;
    }
    pub fn set_tipoMascota(&mut self,tipoMascota: String) {
        self.tipoMascota = tipoMascota;
    }
    pub fn get_nombre(self) -> String {
        self.nombre
    }
    pub fn get_tipoMascota(self) -> String {
        self.tipoMascota
    }

}
impl Display for Mascota {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"Nombre: {}\tTipo de mascota: {}\n",self.nombre, self.tipoMascota)
    }
}