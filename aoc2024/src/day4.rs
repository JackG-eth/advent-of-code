use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
	branch::alt, bytes::complete::tag, character::complete::{anychar, char, digit1}, combinator::value, error::{Error, ErrorKind}, multi::{many0, many1}, sequence::tuple, Err, IResult
};

#[aoc_generator(day4)]
fn parse(input: &str) -> String {
   input.to_string()
}

#[aoc(day4, part1)]
fn part1(input: &str) -> u32 {
    0
}

#[aoc(day4, part2)]
fn part2(input: &str) -> u32 {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";
    const INPUTP2: &str = "";

	#[test]
	fn test1() {
		assert_eq!(part1(&parse(INPUT)), 161);
	}

    #[test]
	fn test2() {
		assert_eq!(part2(&parse(INPUTP2)), 48);
	}
}
