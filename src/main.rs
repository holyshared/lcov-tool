mod coverage;
mod report;
mod parser;
mod processor;

extern crate clap;
extern crate lcov_parser;

use clap:: { App, SubCommand, Arg };

fn main() {
    let file = Arg::with_name("file")
        .value_name("FILE")
        .help("The report file of LCOV");
    let coverage = SubCommand::with_name("coverage")
        .arg(file);

    let app = App::new("lcovtool")
        .version("1.0")
        .author("Noritaka Horio <holy.shared.design@gmail.com>")
        .about("LCOV report of utility tool")
        .subcommand(coverage);

    let matches = app.get_matches();

    match matches.subcommand_name() {
        Some("coverage") => { println!("{:?}", matches); },
        _ => { }
    }
}
