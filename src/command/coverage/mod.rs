mod parser;
mod processor;

use std::io:: { Error };
use std::result:: { Result };
use std::convert:: { From };
use std::fmt:: { Display, Formatter, Error as FormatError };
use lcov_parser:: { ParseError as LCOVParseError, RecordParseError };
use clap:: { App, Arg, ArgMatches, SubCommand, AppSettings };

use command::coverage::parser:: { ReportParser };

pub enum CoverageError {
    IOError(Error),
    ParseError(RecordParseError)
}

impl From<RecordParseError> for CoverageError {
    fn from(error: RecordParseError) -> Self {
        CoverageError::ParseError(error)
    }
}

impl From<Error> for CoverageError {
    fn from(error: Error) -> Self {
        CoverageError::IOError(error)
    }
}

impl From<LCOVParseError> for CoverageError {
    fn from(error: LCOVParseError) -> Self {
        match error {
            LCOVParseError::IOError(err) => CoverageError::IOError(err),
            LCOVParseError::RecordParseError(err) => CoverageError::ParseError(err)
        }
    }
}

impl Display for CoverageError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), FormatError> {
        match self {
            &CoverageError::IOError(ref error) => write!(formatter, "{}", error),
            &CoverageError::ParseError(ref error) => write!(formatter, "{}", error),
        }
    }
}

pub fn coverage_parser<'a, 'b>() -> App<'a, 'b> {
    let file = Arg::with_name("report")
        .value_name("FILE")
        .help("The report file of LCOV");

    SubCommand::with_name("coverage")
        .about("Display code coverage of report file")
        .setting(AppSettings::ColoredHelp)
        .arg(file)
}

pub fn coverage_action<'a>(args: &ArgMatches<'a>) -> Result<(), CoverageError> {
    let report_file = args.value_of("report").unwrap_or("coverage.lcov");
    let mut parser = try!(ReportParser::from_file(report_file));
    let report = try!(parser.parse());



//    let mut parser = ReportParser::new(&content[..]);

    println!("\nCoverage report of file\n");
    println!("{}", report);
    Ok(())
}
/*
fn read_report_from(path: &str) -> IOResult<String> {
    let mut file = try!(File::open(path));
    let mut buffer = String::new();
    let _ = try!(file.read_to_string(&mut buffer));
    Ok(buffer.clone())
}
*/
