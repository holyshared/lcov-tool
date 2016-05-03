use std::path:: { Path, PathBuf };
use std::fmt:: { Display, Formatter, Result };
use coverage:: { Coverage };

#[derive(Debug)]
pub struct Report {
    files: Vec<FileResult>
}

impl Report {
    pub fn new(files: Vec<FileResult>) -> Self {
        Report { files: files }
    }
    pub fn files(&self) -> Vec<FileResult> {
        self.files.to_vec()
    }
    pub fn sorted_files(&self) -> Vec<FileResult> {
        let mut files = self.files();
        files.sort_by(|a, b| { b.coverage.partial_cmp(&a.coverage).unwrap() });
        files
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for file in self.sorted_files().iter() {
            let _ = try!(write!(f, "{}", file));
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct FileResult {
    path: PathBuf,
    coverage: Coverage
}

impl FileResult {
    pub fn new<P: AsRef<Path>>(path: P, coverage: Coverage) -> Self {
        FileResult { path: path.as_ref().to_path_buf(), coverage: coverage }
    }
    pub fn coverage(&self) -> &Coverage {
        &self.coverage
    }
}

impl Display for FileResult {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "{} {}", self.coverage(), self.path.display())
    }
}

#[cfg(test)]
mod tests {
    use report:: { Report, FileResult };
    use coverage:: { Coverage };

    #[test]
    fn test_sorted_files() {
        let report = Report::new(vec!(
            FileResult::new("test1.rs", Coverage::new(0.1)),
            FileResult::new("test2.rs", Coverage::new(0.2)),
            FileResult::new("test3.rs", Coverage::new(0.3))
        ));
        let files = report.sorted_files();

        assert_eq!(&Coverage::new(0.3), files.get(0).unwrap().coverage());
        assert_eq!(&Coverage::new(0.2), files.get(1).unwrap().coverage());
        assert_eq!(&Coverage::new(0.1), files.get(2).unwrap().coverage());
    }

    #[test]
    fn test_display_file_result() {
        let result = FileResult::new("test1.rs", Coverage::new(0.1));
        println!("{}", result);
    }
}
