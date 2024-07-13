use clap::{Error, Parser};

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    message: String,
    #[arg(short, long)]
    uppercase: bool,
    #[arg(short, long)]
    lowercase: bool,
    #[arg(short, long)]
    reverse: bool,
}

fn main() {
    let args: Args = Args::parse();
    let mut message = args.message.clone();

    if args.uppercase {
        message = message.to_uppercase();
    }

    if args.lowercase {
        message = message.to_lowercase();
    }

    if args.reverse {
        message = message.chars().rev().collect();
    }

        println!("{}", message);
}
