/// # Arguments
/// * `left` - The left vector of numbers
/// * `right` - The right vector of numbers
/// # Returns
/// * The total distance between the two vectors  
/// # Panics
/// * Panics if the two vectors are not the same length
fn solve_distances(mut left: Vec<u64>, mut right: Vec<u64>) -> u64 {
    left.sort();
    right.sort();
    left.iter().zip(right.iter()).map(|(l, r)| l.abs_diff(*r)).sum()
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::solve_distances;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    #[test]
    fn part_one() -> io::Result<()> {
        let path = Path::new("input.txt"); // Specify the path to your file
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        let mut column1: Vec<u64> = Vec::new();
        let mut column2: Vec<u64> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let numbers: Vec<u64> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            if numbers.len() == 2 {
                column1.push(numbers[0]);
                column2.push(numbers[1]);
            }
        }

        // You can now use column1 and column2 as needed
        assert!(!column1.is_empty());
        assert!(!column2.is_empty());

        let distance = solve_distances(column1, column2);
        println!("Distance: {}", distance);

        Ok(())
    }
}
