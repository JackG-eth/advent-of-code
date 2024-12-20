use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Vec<char>>{
    input.lines().map(|line| {
            line.chars().collect()
        }
    ).collect()
}

#[aoc(day7, part1)]
fn part1(input: &[Vec<char>]) -> u64 {

}

#[aoc(day7, part2)]
fn part2(input: &[Vec<char>]) -> u64 {

}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "";

	#[test]
	fn test1() {
		assert_eq!(part1(&parse(INPUT)), 41);
	}

	#[test]
	fn test2() {
		assert_eq!(part2(&parse(INPUT)), 6);
	}
}