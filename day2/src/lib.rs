/// A report is a list of levels each separated by a space
pub struct Report {
    levels: Vec<u64>,
}
/// A collection of reports
pub struct Reports {
    reports: Vec<Report>,
}

impl Report {
    pub fn new(levels: Vec<u64>) -> Self {
        Report { levels }
    }

    // todo
    pub fn is_safe(&self) -> bool {
        false
    }
}

impl Reports {
    pub fn new(reports: Vec<Report>) -> Self {
        Reports { reports }
    }

    // todo
    pub fn count_safe(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    fn setup() -> Result<Reports, io::Error> {
        let path = Path::new("input.txt"); // Specify the path to your file
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        let mut list_of_reports: Vec<Report> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let numbers: Vec<u64> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            list_of_reports.push(Report::new(numbers));
        }

        let reports = Reports::new(list_of_reports);

        Ok(reports)
    }
}
