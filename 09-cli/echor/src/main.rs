use clap::{Arg, Command};

fn main() {
    let _matches = Command::new("echor")
        .version("0.1.0")
        .author("qianyan <qianyan.lambda@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short('n')
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

        let text: Vec<&str>  = _matches.get_many::<String>("text").unwrap().map(|s| s.as_str()).collect();
        let omit_newline = _matches.is_present("omit_newline");

        let ending = if omit_newline { "" } else {"\n"};

    print!("{}{}", text.join(" "), ending);
}
