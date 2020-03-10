use std::path::PathBuf;

mod lexer;
use lexer::Lexer;
mod parser;
use parser::Parser;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "llc")]
struct LLC {
    #[structopt(
        name = "standard",
        long,
        help = "The standard to use when compiling to LC3"
    )]
    standard: Option<u32>,
    #[structopt(name = "FILE", parse(from_os_str), help = "The files to compile")]
    files: Vec<PathBuf>,
}

fn main() {
    let args = LLC::from_args();
    args.files
        .into_iter()
        .for_each(move |file| println!("{:#?}", Parser::from_file(file)));
}
