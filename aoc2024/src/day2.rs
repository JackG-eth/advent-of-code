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
        let is_increasing = self.levels.get(0) > self.levels.get(1);
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

    // 7 6 4 2 1: Safe without removing any level.
    // 1 2 7 8 9: Unsafe regardless of which level is removed.
    // 9 7 6 2 1: Unsafe regardless of which level is removed.
    // 1 3 2 4 5: Safe by removing the second level, 3.
    // 8 6 4 4 1: Safe by removing the third level, 4.
    // 1 3 6 7 9: Safe without removing any level.
    pub fn is_safe_with_dampener(&self) -> bool {
        let is_increasing = self.levels.get(0) > self.levels.get(1);
        let mut count = 0;
        for i in 0..self.levels.len()-1 {
            for j in 1..self.levels.len() {
                if is_increasing {
                    if self.levels[i] < self.levels[j] {
                        if j+1 < self.levels.len()-1 && self.levels[i] > self.levels[j+1] {
                            return false;
                        }
                        count +=1;
                        if count > 1 {
                            return false;
                        }
                    }
                } else {
                    if self.levels[i] > self.levels[j] {
                        if j+1 < self.levels.len()-1 && self.levels[i] > self.levels[j+1] {
                            return false;
                        }
                        count +=1;
                        if count > 1 {
                            return false;
                        }
                    }
                }
                let abs_diff = self.levels[i].abs_diff(self.levels[i]);
                if !(abs_diff >= 1 && abs_diff <=3){
                    return false;
                }
            }
        }
        true
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

    
    pub fn count_safe(&self) -> usize {
        self.reports.iter().filter(|r| r.is_safe()).count()
    }

    pub fn count_safe_dampner(&self) -> usize {
        self.reports.iter().filter(|r| r.is_safe_with_dampener()).count()
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

    // 7 6 4 2 1: Safe without removing any level.
    // 1 2 7 8 9: Unsafe regardless of which level is removed.
    // 9 7 6 2 1: Unsafe regardless of which level is removed.
    // 1 3 2 4 5: Safe by removing the second level, 3.
    // 8 6 4 4 1: Safe by removing the third level, 4.
    // 1 3 6 7 9: Safe without removing any level.
    #[test]
    fn test_reports_dampner() {
        let report =  Report { levels: [7,6,4,2, 1].to_vec() };
        let report1 =  Report { levels: [1 ,2, 7 ,8 ,9].to_vec() };
        let report2 =  Report { levels: [9,7,6,2 ,1].to_vec() };
        let report3 =  Report { levels: [1,3,2 ,4 ,5].to_vec() };
        let report4 =  Report { levels: [8,6 ,4 ,4 ,1].to_vec() };
        let report5 =  Report { levels: [1 ,3 ,6 ,7 ,9].to_vec() };

        let reports = Reports {
            reports: vec![report,report1,report2,report3,report4,report5],
        };


        
        let count = reports.count_safe_dampner();

        assert_eq!(count, 4);
    }
    
    #[test]
    fn test_reports_dampner_count() {
        let reports = setup().unwrap();
        
        let count = reports.count_safe_dampner();

        println!("Count: {}", count);
    }
    
}
