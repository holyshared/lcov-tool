mod coverage;
mod report;
mod parser;
mod processor;
mod command;

extern crate clap;
extern crate lcov_parser;

use clap:: { App };
use command:: { coverage_parser, coverage_action };

fn main() {
    let app = App::new("lcovtool")
        .version("1.0")
        .author("Noritaka Horio <holy.shared.design@gmail.com>")
        .about("LCOV report of utility tool")
        .subcommand(coverage_parser());

    let matches = app.get_matches();

    match matches.subcommand() {
        ("coverage", Some(args)) => {
            match coverage_action(args) {
                Ok(_) => { println!("ok"); },
                Err(err) => { println!("{}", err); }
            };
        },
        _ => { }
    }
}
