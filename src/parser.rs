use std::result:: { Result };
use lcov_parser:: { LCOVParser, LCOVRecord, RecordParseError };
use report:: { Report, FileResult };
use coverage:: { Coverage };

pub struct ReportParser {
    parser: LCOVParser,
    files: Vec<FileResult>,
    counter: Option<HitCounter>
}

impl ReportParser {
    pub fn new(report: &str) -> Self {
        let parser = LCOVParser::new(report);
        ReportParser { parser: parser, files: vec!(), counter: None }
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
        self.counter = Some(HitCounter::new(name));
    }

    pub fn on_data(&mut self, excution_count: u32) {
        println!("{}", excution_count);
        match self.counter.as_mut() {
            Some(counter) => counter.proceed(excution_count),
            None => {}
        }
    }

    pub fn on_end_of_record(&mut self) {
        println!("{}", "excution_count");
        match self.counter.as_mut() {
            Some(counter) => self.files.push(FileResult::new(counter.name(), Coverage::new(0.1))),
            None => {}
        }
    }
}

pub struct HitCounter {
    name: String,
    found: u32,
    hit: u32
}

impl HitCounter {
    pub fn new(name: String) -> Self {
        HitCounter {
            name: name,
            found: 0,
            hit: 0
        }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn proceed(&mut self, excution_count: u32) {
        self.found += 1;
        if excution_count <= 0 {
            return;
        }
        self.hit += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::fs:: { File };
    use parser:: { ReportParser };
    use std::io:: { Read };

    #[test]
    fn test_parse_report() {
        let mut buffer = String::new();
        let mut f = File::open("tests/fixtures/report.lcov").unwrap();
        let _ = f.read_to_string(&mut buffer);

        let mut parser = ReportParser::new(buffer.as_str());
        let report = parser.parse().unwrap();
        assert_eq!(report.files().len(), 2);
    }
}
