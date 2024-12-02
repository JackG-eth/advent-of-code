use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
	let mut a = Vec::with_capacity(1500);
	let mut b = Vec::with_capacity(1500);
	for l in input.lines() {
		let mut iter = l.split_whitespace();
		a.push(iter.next().unwrap().parse().unwrap());
		b.push(iter.next().unwrap().parse().unwrap());
	}
	a.sort_unstable();
	b.sort_unstable();
	(a, b)
}

/// # Arguments
/// * `left` - The left vector of numbers
/// * `right` - The right vector of numbers
/// # Returns
/// * The total distance between the two vectors  
/// # Panics
/// * Panics if the two vectors are not the same length, assumes they are also sorted.
#[aoc(day1, part1)]
fn part1(input: &(Vec<i32>, Vec<i32>)) -> u32 {
    let (left,right) = input;
    left.iter().zip(right.iter()).map(|(l, r)| l.abs_diff(*r)).sum()
}

/// # Arguments
/// * `left` - The left vector of numbers
/// * `right` - The right vector of numbers
/// # Returns
/// * The similarity score between the two vectors
/// # Assumptions
/// * The vectors are sorted and of the same length
#[aoc(day1, part2)]
fn part2(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let (left,right) = input;
    left.iter().map(|left|  left * right.iter().filter(|&right| *right == *left).count() as i32).sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3";

	#[test]
	fn test1() {
		assert_eq!(part1(&parse(INPUT)), 11);
	}

	#[test]
	fn test2() {
		assert_eq!(part2(&parse(INPUT)), 31);
	}
}
