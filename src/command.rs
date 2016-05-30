use lcov_parser:: { RecordParseError };
use parser:: { ReportParser };
use std::result:: { Result };

pub fn coverage(report: &str) -> Result<(), RecordParseError> {
    let mut parser = ReportParser::new(report);

    match parser.parse() {
        Ok(report) => {
            println!("{}", report);
            Ok(())
        },
        Err(error) => Err(error)
    }
}
