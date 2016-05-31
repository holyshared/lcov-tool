mod coverage;
mod report;
mod parser;
mod processor;
mod command;

extern crate clap;
extern crate lcov_parser;

use clap:: { App, SubCommand, Arg };
use command:: { coverage };

fn main() {
    let file = Arg::with_name("file")
        .value_name("FILE")
        .help("The report file of LCOV");
    let coverage_command = SubCommand::with_name("coverage")
        .arg(file);

    let app = App::new("lcovtool")
        .version("1.0")
        .author("Noritaka Horio <holy.shared.design@gmail.com>")
        .about("LCOV report of utility tool")
        .subcommand(coverage_command);

    let matches = app.get_matches();

    match matches.subcommand() {
        ("coverage", Some(args)) => {
            match coverage(args) {
                Ok(_) => { println!("ok"); },
                Err(err) => { println!("{}", err); }
            };
        },
        _ => { }
    }
}
