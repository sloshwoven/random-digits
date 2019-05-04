extern crate clap;
extern crate rand;

use clap::{App, Arg, ArgMatches};
use rand::Rng;

fn main() {
    let matches = parse_args();

    // these unwraps are safe because parse_args would have failed if either wasn't present or
    // wasn't formatted correctly
    let lines = matches.value_of("lines").unwrap().parse::<u32>().unwrap();
    let width = matches.value_of("width").unwrap().parse::<u32>().unwrap();

    print_random_digits(lines, width);
}

fn parse_args<'a>() -> ArgMatches<'a> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("lines")
                .required(true)
                .takes_value(true)
                .value_name("LINES")
                .validator(is_valid_num)
                .help("number of lines of output to produce"),
        )
        .arg(
            Arg::with_name("width")
                .required(true)
                .takes_value(true)
                .value_name("WIDTH")
                .validator(is_valid_num)
                .help("number of random digits on each output line"),
        )
        .get_matches()
}

fn is_valid_num(s: String) -> Result<(), String> {
    s.parse::<u32>().map(|_| ()).map_err(|_| {
        format!(
            "{} is not an integer between {} and {}",
            s,
            u32::min_value(),
            u32::max_value()
        )
    })
}

fn print_random_digits(lines: u32, width: u32) {
    let mut rng = rand::thread_rng();

    for _ in 0..lines {
        for _ in 0..width {
            print!("{}", rng.gen_range(0, 10));
        }

        println!();
    }
}
