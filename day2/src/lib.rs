/// The size of the window to check for adjacent levels
pub const WINDOW_SIZE: usize = 2;

/// A report is a list of levels each separated by a space
pub struct Report {
    levels: Vec<u64>,
}

impl Report {
    pub fn new(levels: Vec<u64>) -> Self {
        Report { levels }
    }

    // todo
    /// The levels are either all increasing or all decreasing.
    /// Any two adjacent levels differ by at least one and at most three.
    pub fn is_safe(&self) -> bool {
        let is_increasing = self.levels.get(0) > self.levels.get(2);
        self.levels.windows(WINDOW_SIZE).all(|w| {
            if is_increasing {
                if w[0] < w[1] {
                    return false;
                }
            } else {
                if w[0] > w[1] {
                    return false;
                }
            }
            let abs_diff = w[0].abs_diff(w[1]);
            abs_diff >= 1 && abs_diff <= 3
        })
    }
}

/// A collection of reports
pub struct Reports {
    reports: Vec<Report>,
}

impl Reports {
    pub fn new(reports: Vec<Report>) -> Self {
        Reports { reports }
    }

    // todo
    pub fn count_safe(&self) -> usize {
        self.reports.iter().filter(|r| r.is_safe()).count()
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

    #[test]
    fn test_is_safe() {
        let report = Report::new(vec![1, 2, 3, 4]);
        assert!(report.is_safe());
    }

    #[test]
    fn test_is_safe_decreasing() {
        let report = Report::new(vec![4, 3, 2, 1]);
        assert!(report.is_safe());
    }

    #[test]
    fn test_reports() {
        let reports = setup().unwrap();
        
        let count = reports.count_safe();

        println!("Count: {}", count);
    }
}
