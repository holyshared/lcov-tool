use std::fs:: { File };
use std::io:: { Read, Error, Result as IOResult };
use std::result:: { Result };
use std::convert:: { From };
use std::fmt:: { Display, Formatter, Error as FormatError };
use lcov_parser:: { RecordParseError };
use parser:: { ReportParser };
use clap:: { ArgMatches };

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

impl Display for CoverageError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), FormatError> {
        match self {
            &CoverageError::IOError(ref error) => write!(formatter, "{}", error),
            &CoverageError::ParseError(ref error) => write!(formatter, "{}", error),
        }
    }
}

pub fn coverage<'a>(args: &ArgMatches<'a>) -> Result<(), CoverageError> {
    let report_file = args.value_of("report").unwrap_or("coverage.lcov");
    let content = try!(read_report_from(report_file));

    let mut parser = ReportParser::new(&content[..]);
    let report = try!(parser.parse());

    println!("\nCoverage report of file\n");
    println!("{}", report);
    Ok(())
}

fn read_report_from(path: &str) -> IOResult<String> {
    let mut file = try!(File::open(path));
    let mut buffer = String::new();
    let _ = try!(file.read_to_string(&mut buffer));
    Ok(buffer.clone())
}
