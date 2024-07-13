use std::fs::read_dir;
use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(author, about, long_about = None, version)]
struct Args {
    path: String,
    #[arg(short, long)]
    subdirectories: bool,
    #[arg(short, long)]
    all: bool, // Esto mostraría el tipo de fichero (Carpeta, Archivo). Pendiente de implementacion
}

fn main() {
    let args = Args::parse();
    let path: String = args.path;

    if !args.subdirectories {
        for p in read_dir(&path).unwrap() {
            println!("{}", p.unwrap().path().display());
        }
    } else {
        for p in read_dir(&path).unwrap() { //p es la carpeta padre (desde la que se inicia)
            match p {
                Ok(p) => {
                    if p.metadata().unwrap().is_dir() {
                        for sp in read_dir(p.path()).unwrap() { //sp es cada entrada del subdirectorio
                            match sp {
                                Ok(sp) => println!("{}", sp.path().display()),
                                Err(_) => eprintln!("Error leyendo subentrada"),
                            }
                        }
                    } else {
                        println!("{}", p.path().display());
                    }
                },
                Err(_) => eprintln!("Error reading entry"),
            }
        }
    }


}
