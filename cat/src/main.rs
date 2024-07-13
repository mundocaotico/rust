use clap::{Error, Parser};
use std::{fmt::Display, fs::File, io::{BufReader, Read}};
use encoding_rs::Encoding;
use encoding_rs_io::DecodeReaderBytesBuilder;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    //#[arg(short, long)]
    name: String,
    #[arg(short, long)]
    reverse: bool
}


fn main() {
    //Inicializamos los argumentos de la estructura
    let args = Args::parse();
    let fichero: File;
    let mut buf: String = String::new();
    // Variable que se usa para señalar si se encontro o no el fichero
    let resultado: Result<File, Error> = match File::open(&args.name) {
        Ok(file) => Ok(file),
        Err(_) => {
            println!("No se ha encontrado el fichero");
            return;
        }
    };
    fichero = resultado.unwrap();

    // Variable que se usa para mostrar el contenido del fichero
    let mut reader = DecodeReaderBytesBuilder::new()
    .encoding(Some(Encoding::for_label(b"utf-8").unwrap()))
    .build(BufReader::new(fichero));

    // reader hace referencia al archivo abierto, leemos el contenido y lo guardamos en la variable buf
    // La manera en la que trata los encodings es la siguiente:
    // - Si no se especifica ningún encoding, se intenta detectar el encoding automáticamente
    // - Si se especifica un encoding, se usa ese encoding
    // - Si el encoding detectado no es compatible con el encoding especificado, se lanza un error

    // Leemos el fichero y lo guardamos en la variable buf
    reader.read_to_string(&mut buf).unwrap();

    //println!("{}", buf);
    
    if !args.reverse {
        for linea in buf.lines() {
            println!("{}", linea);
        }
    } else {
        for linea in buf.lines().rev() {
            println!("{}", linea)
        }
    }

}
