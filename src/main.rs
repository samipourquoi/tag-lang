#![allow(dead_code)]
#![allow(unused_imports)]

use nom_greedyerror::convert_error;
use nom::Err::*;
use crate::errors::CompilerError;
use std::io;
use std::io::BufRead;
use clap::{App, SubCommand, Arg};
use std::path::{Path, PathBuf};
use std::error::Error;
use nom::Finish;

mod parser;
mod generator;
mod errors;

pub const FILE_EXTENSION: &str = ".tag";

pub const BOOTSTRAP: &str = include_str!("./data/bootstrap.tag");

pub struct CompileOptions {
    pub outdir: PathBuf,
    pub namespace: String
}

fn main() {
    let matches = App::new("Tag Compiler")
        .version(env!["CARGO_PKG_VERSION"])
        .author(env!["CARGO_PKG_AUTHORS"])
        .about(env!["CARGO_PKG_DESCRIPTION"])
        .subcommand(SubCommand::with_name("compile")
            .about("Compile a Tag source file to a datapack")
            .arg(Arg::with_name("FILE")
                .help("The input file to compile")
                .validator(|f|
                    if f.ends_with(FILE_EXTENSION) {
                        Ok(())
                    } else {
                        Err(format!("File must have the {} file extension", FILE_EXTENSION))
                    }
                )
                .required(true))
            .arg(Arg::with_name("namespace")
                .help("Set the namespace of the datapack")
                .takes_value(true)
                .short("n")
                .long("name"))
            .arg(Arg::with_name("outdir")
                .help("Directory in which the datapack will be generated")
                .takes_value(true)
                .short("o")
                .long("outdir")))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("compile") {
        let file = matches.value_of("FILE").unwrap();
        let namespace = matches.value_of("namespace").map_or(
            PathBuf::from(file).file_stem().unwrap().to_str().unwrap().to_string(),
            ToString::to_string
        );
        let outdir = matches.value_of("outdir")
            .unwrap_or(&file[0..file.len() - FILE_EXTENSION.len()]);
        let input = match std::fs::read_to_string(file) {
            Ok(input) => input,
            Err(err) => clap::Error::with_description(err.to_string().as_str(), clap::ErrorKind::ValueValidation)
                .exit()
        };
        let input = input.as_str();
        let options = CompileOptions {
            outdir: PathBuf::from(outdir),
            namespace: namespace.to_string()
        };

        let result = compile(input, options);

        if let Ok(result) = result {
        } else if let Err(err) = result {
            err.format(input);
        }
    }

    // let stdin = io::stdin();
    // let lines: String = stdin.lock().lines()
    //     .filter_map(|x| x.ok())
    //     .collect::<Vec<String>>()
    //     .join("\n") + "\n";
    // let input = lines.as_str();
}

fn compile(input: &str, options: CompileOptions) -> Result<(), CompilerError> {
    let (_, bootstrap) = parser::parse(BOOTSTRAP).finish()?;
    let (_, ast) = parser::parse(input).finish()?;
    let ast = bootstrap + ast;
    dbg!(&ast);
    generator::generate(ast, options)
}
