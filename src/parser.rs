use std::io:: { Read, Result };
use lcov_parser:: { LCOVParser, LCOVRecord, ParsedResult };
use report:: { Report, FileResult };
use coverage:: { Coverage };

pub struct ReportParser<R> {
    parser: LCOVParser<R>
}

impl<R: Read> ReportParser<R> {
    pub fn new(reader: R) -> Self {
        let parser = LCOVParser::new(reader);
        ReportParser { parser: parser }
    }
    pub fn parse(&mut self) -> Result<Report> {
        loop {
            match self.parser.parse_next() {
                ParsedResult::Eof => { break; },
                ParsedResult::Ok(record, _) => self.on_parsed(record),
                ParsedResult::Err(error) => panic!("{:?}", error)
            }
        }

        let report = Report::new(vec!(
            FileResult::new("test1.rs", Coverage::new(0.1)),
            FileResult::new("test2.rs", Coverage::new(0.2)),
            FileResult::new("test3.rs", Coverage::new(0.3))
        ));
        Ok(report)
    }

    pub fn on_parsed(&self, record: LCOVRecord) {
        match record {
            LCOVRecord::SourceFile(name) => self.on_source_file(name),
            LCOVRecord::Data(_, excution_count, _) => self.on_data(excution_count),
            LCOVRecord::EndOfRecord => self.on_end_of_record(),
            _ => {}
        }
    }

    pub fn on_source_file(&self, name: String) {
        println!("{}", name);
    }

    pub fn on_data(&self, excution_count: u32) {
        println!("{}", excution_count);
    }

    pub fn on_end_of_record(&self) {
        println!("{}", "excution_count");
    }
}

#[cfg(test)]
mod tests {
    use std::fs:: { File };
    use parser:: { ReportParser };

    #[test]
    fn test_parse_report() {
        let f = File::open("tests/fixtures/report.lcov").unwrap();
        let mut parser = ReportParser::new(f);
        let report = parser.parse().unwrap();
        assert_eq!(report.files().len(), 2);
    }
}
