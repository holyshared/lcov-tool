use std::result:: { Result };

use std::fs:: { File };
use std::io:: { Result as IOResult };
use std::path:: { Path };

use lcov_parser:: { LCOVParser, LCOVRecord, LineData, FromFile, ParseError };
use report:: { Report, FileResult };
use command::coverage::processor:: { FileProcessor, ToFileResult };

pub struct ReportParser {
    files: Vec<FileResult>,
    parser: LCOVParser<File>,
    processor: Option<FileProcessor>
}

impl ReportParser {
    pub fn new(parser: LCOVParser<File>) -> Self {
        ReportParser {
            parser: parser,
            processor: None,
            files: vec!(),
        }
    }
    pub fn from_file<P: AsRef<Path>>(path: P) -> IOResult<Self> {
        let parser = try!(LCOVParser::from_file(path));
        Ok(ReportParser::new(parser))
    }
    pub fn parse(&mut self) -> Result<Report, ParseError> {
        loop {
            let result = try!(self.parser.next());

            match result {
                Some(ref record) => self.on_parsed(record),
                None => { break; }
            }
        }
        let report = Report::new(self.files.clone());
        Ok(report)
    }

    pub fn on_parsed(&mut self, record: &LCOVRecord) {
        match record {
            &LCOVRecord::SourceFile(ref name) => self.on_source_file(name.clone()),
            &LCOVRecord::Data(ref data) => self.on_data(data),
            &LCOVRecord::EndOfRecord => self.on_end_of_record(),
            _ => {}
        }
    }

    pub fn on_source_file(&mut self, name: String) {
        self.processor = Some(FileProcessor::new(name));
    }

    pub fn on_data(&mut self, data: &LineData) {
        match self.processor.as_mut() {
            Some(processor) => processor.proceed(data.count),
            None => {}
        }
    }

    pub fn on_end_of_record(&mut self) {
        match self.processor.as_mut() {
            Some(processor) => self.files.push(processor.to_file_result()),
            None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use command::coverage::parser:: { ReportParser };
    use coverage:: { Coverage };

    #[test]
    fn test_parse_report() {
        let mut parser = ReportParser::from_file("tests/fixtures/fixture.info").unwrap();
        let report = parser.parse().unwrap();
        let files = report.files();

        assert_eq!(files.len(), 4);

        let first = files.get(0).unwrap();
        assert_eq!(first.coverage(), &Coverage::new(1.0));

        let second = files.get(1).unwrap();
        assert_eq!(second.coverage(), &Coverage::new(1.0));

        let third = files.get(2).unwrap();
        assert_eq!(third.coverage(), &Coverage::new(0.8));

        let third = files.get(3).unwrap();
        assert_eq!(third.coverage(), &Coverage::new(0.0));
    }
}
