use std::fmt;
use std::fmt::{Display, Formatter};

pub struct Nota {
    id: u32,
    prioridad: i8,
    texto: String
}

impl Nota {

    pub unsafe fn new() -> Self {
        static mut contador: u32 = 1;
        let id = contador;
        contador += 1;
        Self {
            id,
            prioridad: 0,
            texto: String::new()
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_prioridad(&self) -> i8 {
        self.prioridad
    }

    pub fn get_texto(&self) -> &str {
        self.texto.as_str()
    }

    pub fn set_prioridad(&mut self, prioridad: i8) {
        self.prioridad = prioridad;
    }

    pub fn set_texto(&mut self, texto: String) {
        self.texto = texto;
    }

    pub fn add_nota(&self, notas: &mut Vec<Nota>) {
        notas.push(Nota {
            id: self.id,
            prioridad: self.prioridad,
            texto: String::from(self.get_texto()),
        });
    }

}

impl Display for Nota {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Ok(write!(f, "ID: {} Prioridad: {} Texto: {}", self.get_id(), self.get_prioridad(), self.get_texto()).expect("Error al mostrar los datos"))
    }
}