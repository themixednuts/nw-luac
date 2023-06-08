use clap::parser::ValueSource;
use clap::{Arg, Command};
use std::path::PathBuf;
use std::{env, fs};

pub struct Args {
    pub input: PathBuf,
    pub output: PathBuf,
    pub luac: PathBuf,
}

pub fn run() -> Args {
    let matches = Command::new("nwluac")
        .version("0.1.0")
        .author("Mixed Nuts")
        .about("Compiles Lua(v5.1) and Inserts 04 00 to the beginning two bytes")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("Input PATH")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output PATH")
                .required(false),
        )
        .arg(
            Arg::new("luac")
                .short('l')
                .long("luac")
                .help("luac.exe PATH")
                .required(false),
        )
        .get_matches();

    let input = PathBuf::from(matches.get_one::<String>("input").unwrap());
    let output = match matches.value_source("output") {
        Some(ValueSource::CommandLine) => {
            PathBuf::from(matches.get_one::<String>("output").unwrap())
        }
        Some(_) | None => {
            let mut output = input.clone();
            output.set_extension("luac");
            output
        }
    };
    let luac = match matches.value_source("luac") {
        Some(ValueSource::CommandLine) => PathBuf::from(matches.get_one::<String>("luac").unwrap()),
        Some(_) | None => {
            let target_file = "luac5.1.exe";
            let mut path_buf = PathBuf::new();
            let exe_dir = env::current_exe().expect("Failed to get current exe path");
            let exe_par = exe_dir
                .parent()
                .expect("Current exe has no parent directory");

            for entry in fs::read_dir(exe_par).expect("Failed to read directory") {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if file_name == target_file {
                            path_buf = entry.path();
                            break;
                        }
                    }
                }
            }
            path_buf
        }
    };

    Args {
        input,
        output,
        luac,
    }
}
