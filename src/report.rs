use std::path:: { PathBuf };
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

#[derive(Debug, Clone)]
pub struct FileResult {
    path: PathBuf,
    coverage: Coverage
}

impl FileResult {
    pub fn new(path: PathBuf, coverage: Coverage) -> Self {
        FileResult { path: path, coverage: coverage }
    }
    pub fn coverage(&self) -> &Coverage {
        &self.coverage
    }
}

#[cfg(test)]
mod tests {
    use std::path:: { PathBuf };
    use report:: { Report, FileResult };
    use coverage:: { Coverage };

    #[test]
    fn test_sorted_files() {
        let report = Report::new(vec!(
            FileResult::new(PathBuf::from("test1.rs"), Coverage::new(0.1)),
            FileResult::new(PathBuf::from("test2.rs"), Coverage::new(0.2)),
            FileResult::new(PathBuf::from("test3.rs"), Coverage::new(0.3))
        ));
        let files = report.sorted_files();

        assert_eq!(&Coverage::new(0.3), files.get(0).unwrap().coverage());
        assert_eq!(&Coverage::new(0.2), files.get(1).unwrap().coverage());
        assert_eq!(&Coverage::new(0.1), files.get(2).unwrap().coverage());
    }
}
