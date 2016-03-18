use std::io:: { Read, Result };
use lcov_parser:: { LCOVParser, LCOVRecord, ParsedResult };
use report:: { Report, FileResult };
use coverage:: { Coverage };

pub struct ReportParser<R> {
    parser: LCOVParser<R>,
    files: Vec<FileResult>,
    counter: Option<HitCounter>
}

impl<R: Read> ReportParser<R> {
    pub fn new(reader: R) -> Self {
        let parser = LCOVParser::new(reader);
        ReportParser { parser: parser, files: vec!(), counter: None }
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

    pub fn on_parsed(&mut self, record: LCOVRecord) {
        match record {
            LCOVRecord::SourceFile(name) => self.on_source_file(name),
            LCOVRecord::Data(_, excution_count, _) => self.on_data(excution_count),
            LCOVRecord::EndOfRecord => self.on_end_of_record(),
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

    #[test]
    fn test_parse_report() {
        let f = File::open("tests/fixtures/report.lcov").unwrap();
        let mut parser = ReportParser::new(f);
        let report = parser.parse().unwrap();
        assert_eq!(report.files().len(), 2);
    }
}
