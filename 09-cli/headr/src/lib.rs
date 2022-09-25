use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{value_parser, Arg, Command};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(val.into()),
    }
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let _matches = Command::new("headr")
        .version("0.1.0")
        .author("qianyan qianyan.lambda@gmail.com")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple_values(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
                .short('n')
                .value_parser(value_parser!(usize))
                .takes_value(true)
                .default_value("10")
                .help("print first n lines, 10 by default.")
                .conflicts_with("bytes"),
        )
        .arg(
            Arg::with_name("bytes")
                .short('c')
                .value_parser(value_parser!(usize))
                .takes_value(true)
                .help("print first c bytes"),
        )
        .get_matches();

    let files: Vec<String> = _matches
        .get_many::<String>("files")
        .unwrap()
        .map(|s| s.to_owned())
        .collect();

    let lines = _matches.get_one::<usize>("lines").unwrap().to_owned();
    let bytes = _matches.get_one::<usize>("bytes").map(|b| b.to_owned());

    Ok(Config {
        files,
        lines,
        bytes,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(reader) => {
                for line in reader.lines().take(config.lines) {
                    println!("{}", line?);
                }
            }
        }
    }
    Ok(())
}
