use clap::{Arg, Command};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let _matches = Command::new("catr")
        .version("0.1.0")
        .author("qianyan qianyan.lambda@gmail.com")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple_values(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .short('n')
                .help("Number the output lines, starting at 1.")
                .takes_value(false)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short('b')
                .help("Number the non-blank output lines, starting at 1.")
                .takes_value(false),
        )
        .get_matches();

    let files: Vec<String> = _matches
        .get_many::<String>("files")
        .unwrap()
        .map(|s| s.to_owned())
        .collect();
    let number_lines = _matches.is_present("number_lines");
    let number_nonblank_lines = _matches.is_present("number_nonblank_lines");

    Ok(Config {
        files,
        number_lines,
        number_nonblank_lines,
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
                let mut nonblank_line_num = 0;
                for (line_num, line) in reader.lines().enumerate() {
                    let line = line?;
                    if config.number_lines {
                        println!("{:>6}\t{}", line_num + 1, line)
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!()
                        } else {
                            nonblank_line_num += 1;
                            println!("{:>6}\t{}", nonblank_line_num, line)
                        }
                    } else {
                        println!("{}", line)
                    }
                }
            }
        }
    }
    Ok(())
}
