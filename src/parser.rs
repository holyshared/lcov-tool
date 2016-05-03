use std::result:: { Result };
use lcov_parser:: { LCOVParser, LCOVRecord, RecordParseError };
use report:: { Report, FileResult };
use processor:: { FileProcessor, ToFileResult };

pub struct ReportParser {
    parser: LCOVParser,
    files: Vec<FileResult>,
    processor: Option<FileProcessor>
}

impl ReportParser {
    pub fn new(report: &str) -> Self {
        let parser = LCOVParser::new(report);
        ReportParser { parser: parser, files: vec!(), processor: None }
    }
    pub fn parse(&mut self) -> Result<Report, RecordParseError> {
        let records = try!(self.parser.parse());

        for record in records.iter() {
            self.on_parsed(record);
        }

        let report = Report::new(self.files.clone());
        Ok(report)
    }

    pub fn on_parsed(&mut self, record: &LCOVRecord) {
        match record {
            &LCOVRecord::SourceFile(ref name) => self.on_source_file(name.clone()),
            &LCOVRecord::Data(_, excution_count, _) => self.on_data(excution_count),
            &LCOVRecord::EndOfRecord => self.on_end_of_record(),
            _ => {}
        }
    }

    pub fn on_source_file(&mut self, name: String) {
        println!("{}", name);
        self.processor = Some(FileProcessor::new(name));
    }

    pub fn on_data(&mut self, excution_count: u32) {
        println!("{}", excution_count);
        match self.processor.as_mut() {
            Some(processor) => processor.proceed(excution_count),
            None => {}
        }
    }

    pub fn on_end_of_record(&mut self) {
        println!("{}", "excution_count");
        match self.processor.as_mut() {
            Some(processor) => self.files.push(processor.to_file_result()),
            None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs:: { File };
    use parser:: { ReportParser };
    use coverage:: { Coverage };
    use std::io:: { Read };

    #[test]
    fn test_parse_report() {
        let mut buffer = String::new();
        let mut f = File::open("tests/fixtures/report.lcov").unwrap();
        let _ = f.read_to_string(&mut buffer);

        let mut parser = ReportParser::new(buffer.as_str());
        let report = parser.parse().unwrap();
        let files = report.files();

        assert_eq!(files.len(), 2);

        let first = files.get(0).unwrap();
        assert_eq!(first.coverage(), &Coverage::new(0.75));
    }
}
