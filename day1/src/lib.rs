/// # Arguments
/// * `left` - The left vector of numbers
/// * `right` - The right vector of numbers
/// # Returns
/// * The total distance between the two vectors  
/// # Panics
/// * Panics if the two vectors are not the same length, assumes they are also sorted.
fn solve_distances(left: Vec<u64>, right: Vec<u64>) -> u64 {
    left.iter().zip(right.iter()).map(|(l, r)| l.abs_diff(*r)).sum()
}

/// # Arguments
/// * `left` - The left vector of numbers
/// * `right` - The right vector of numbers
/// # Returns
/// * The similarity score between the two vectors
/// # Assumptions
/// * The vectors are sorted and of the same length
fn similarity_score(left: Vec<u64>, right: Vec<u64>) -> u64 {
    left.iter().map(|left|  left * right.iter().filter(|&right| *right == *left).count() as u64).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solve_distances;
    use crate::similarity_score;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;


    fn setup() -> Result<(Vec<u64>, Vec<u64>), io::Error> {
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

        column1.sort();
        column2.sort();

        Ok((column1, column2))
    }

    #[test]
    fn part_one() -> io::Result<()> {
        let (column1, column2) = setup()?;
        let distance = solve_distances(column1, column2);
        println!("Distance: {}", distance);

        Ok(())
    }

    #[test]
    fn part_two() -> io::Result<()> {
        let (column1, column2) = setup()?;
        let sim_score = similarity_score(column1, column2);
        println!("Similarity Score: {}", sim_score);

        Ok(())
    }
}
