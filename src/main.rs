extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("random-digits")
        .version("0.1.0")
        .arg(Arg::with_name("lines")
            .required(true)
            .takes_value(true)
            .value_name("LINES")
            .validator(is_valid_num)
            .help("number of lines of output to produce"))
        .arg(Arg::with_name("width")
            .required(true)
            .takes_value(true)
            .value_name("WIDTH")
            .validator(is_valid_num)
            .help("number of random digits on each output line"))
        .get_matches();
}

fn is_valid_num(s: String) -> Result<(), String> {
    s.parse::<u32>()
        .map(|_| ())
        .map_err(|_| format!("{} is not a valid number", s))
}
