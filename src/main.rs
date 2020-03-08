use std::fs::*;
use std::io::{Error, Read};
use std::path::PathBuf;

mod lexer;
use lexer::Lexer;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "llc")]
struct LLC {
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() -> Result<(), Error> {
    let args = LLC::from_args();
    args.files.into_iter().for_each(|file| {
        let mut file = File::open(file).unwrap();
        let mut content = String::new();

        file.read_to_string(&mut content).unwrap();

        let lexer = Lexer::new(&content);

        let tokens = lexer.lex();

        println!("{:#?}", tokens);
    });

    Ok(())
}
