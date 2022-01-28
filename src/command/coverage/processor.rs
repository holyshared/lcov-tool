use coverage:: { Coverage };
use report:: { FileResult };

pub struct FileProcessor {
    name: String,
    found: u32,
    hit: u32
}

impl FileProcessor {
    pub fn new(name: String) -> Self {
        FileProcessor {
            name: name,
            found: 0,
            hit: 0
        }
    }
    pub fn proceed(&mut self, excution_count: u32) {
        self.found += 1;
        if excution_count <= 0 {
            return;
        }
        self.hit += 1;
    }
}

pub trait ToFileResult {
    fn to_file_result(&self) -> FileResult;
}

impl ToFileResult for FileProcessor {
    fn to_file_result(&self) -> FileResult {
        let value = match self.found == 0 {
            true => 0_f64,
            false => f64::from(self.hit) / f64::from(self.found),
        };
        FileResult::new(self.name.clone(), Coverage::new(value))
    }
}
