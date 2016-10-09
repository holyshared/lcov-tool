mod coverage;
mod report;
mod command;

extern crate clap;
extern crate lcov_parser;

use clap:: { App, ArgMatches, AppSettings };
use command:: { coverage_parser, coverage_action, CoverageError };
use std::result:: { Result };
use std::convert:: { From };
use std::fmt:: { Display, Formatter, Error as FormatError };

pub enum CommandError {
    Coverage(CoverageError)
}

impl From<CoverageError> for CommandError {
    fn from(error: CoverageError) -> Self {
        CommandError::Coverage(error)
    }
}

impl Display for CommandError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), FormatError> {
        match self {
            &CommandError::Coverage(ref error) => write!(formatter, "{}", error)
        }
    }
}

fn main() {
    let app = App::new("lcovtool")
        .version("1.0")
        .author("Noritaka Horio <holy.shared.design@gmail.com>")
        .about("LCOV report of utility tool")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .subcommand(coverage_parser());

    match run(app.get_matches().subcommand()) {
        Ok(_) => { },
        Err(error) => println!("{}", error)
    }
}

pub fn run<'a>(subcommand: (&str, Option<&ArgMatches<'a>>)) -> Result<(), CommandError> {
    match subcommand {
        ("coverage", Some(args)) => Ok(try!(coverage_action(args))),
        _ => Ok(())
    }
}
