use aoc_runner_derive::{aoc, aoc_generator};

/// The size of the window to check for adjacent levels
pub const WINDOW_SIZE: usize = 2;

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Vec<i32>> {
	let mut reports = Vec::with_capacity(1500);
	for l in input.lines() {
        let report: Vec<i32> = l
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
        reports.push(report);
	}
    reports
}

/// The levels are either all increasing or all decreasing.
/// Any two adjacent levels differ by at least one and at most three.
pub fn is_safe(report: &Vec<i32>) -> bool {
    let is_increasing = report.get(0) > report.get(1);
    report.windows(WINDOW_SIZE).all(|w| {
        if is_increasing {
            if w[0] <= w[1] {
                return false;
            }
        } else {
            if w[0] >= w[1] {
                return false;
            }
        }
        is_valid(&w[0], &w[1])
    })
}

fn is_valid(a: &i32, b: &i32) -> bool {
    let abs_diff = a.abs_diff(*b);
    abs_diff >= 1 && abs_diff <= 3 && a != b
}

/// Super brute force and ugly (will improve later...)
pub fn is_safe_dampner(report: &Vec<i32>) -> bool {
    // Check if the report is already safe
    if is_safe(report) {
        return true;
    }

    // Try removing each level one by one
    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i); // Remove the level at index i

        // Check if the modified report is safe
        if is_safe(&modified_report) {
            return true; // If safe after removal, count as safe
        }
    }

    false // If no removal makes it safe, return false
}


#[aoc(day2, part1)]
fn part1(input: &Vec<Vec<i32>>) -> i32 {
    input.iter().map(|r| (is_safe(r) == true) as i32).sum()
}


#[aoc(day2, part2)]
fn part2(input: &Vec<Vec<i32>>) -> i32 {
    input.iter().map(|r| (is_safe_dampner(r) == true) as i32).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9\n7 10 8 10 11\n29 28 27 25 26 25 22 20";

	#[test]
	fn test1() {
		assert_eq!(part1(&parse(INPUT)), 2);
	}


    // 404?
    #[test]
	fn test2() {
		assert_eq!(part2(&parse(INPUT)), 6);
	}
    
  
}
