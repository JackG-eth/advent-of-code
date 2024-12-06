use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
	branch::alt, bytes::complete::tag, character::complete::{anychar, char, digit1}, combinator::value, error::{Error, ErrorKind}, multi::{many0, many1}, sequence::tuple, Err, IResult
};

/*
 * When to Use a Parser vs. Regex:
 *
 * Use a Parser (e.g., nom):
 * - When dealing with complex or nested data structures (e.g., programming languages, JSON).
 * - When you need structured output and type safety (e.g., defining specific types for different parts of the input).
 * - When you require robust error handling and detailed error reporting.
 * - When you want to build complex parsers from simpler components (composability).
 * - When performance is a concern, especially with large inputs or complex patterns.
 * - When readability and maintainability of the code are priorities.
 *
 * Use Regex:
 * - For simple string searches or replacements (e.g., finding a substring).
 * - For validating simple formats (e.g., email addresses, phone numbers).
 * - For one-off tasks that don't require the complexity of a full parser.
 * - When you need a quick solution and the patterns are straightforward.
 */

#[aoc_generator(day3)]
fn parse(input: &str) -> String {
   input.to_string()
}

#[derive(Debug, Clone)]
enum State {
	Mul(u32),
	Do,
	Dont,
	Garbage,
}

fn mul(i: &str) -> IResult<&str, State> {
	let (remainder, (_, a, _, b, _)) =
		tuple((tag("mul("), digit1, char(','), digit1, char(')')))(i)?;
	if a.len() > 3 || b.len() > 3 {
		return Err(Err::Error(Error::new(i, ErrorKind::Tag)));
	}
	Ok((
		remainder,
		State::Mul(a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap()),
	))
}
#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
	input
		.match_indices("mul")
		.map(|(x, _)| {
			let y = &input[x..];
			if let Ok((_, State::Mul(m))) = mul(y) {
				m
			} else {
				0
			}
		})
		.sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
	let mut skip = false;
	many1(alt((
		mul,
		value(State::Do, tag("do()")),
		value(State::Dont, tag("don't()")),
		value(State::Garbage, anychar),
	)))(input)
	.unwrap()
	.1
	.into_iter()
	.map(|a| match a {
		State::Mul(_) if skip => 0,
		State::Mul(m) => m,
		State::Do => {
			skip = false;
			0
		}
		State::Dont => {
			skip = true;
			0
		}
		State::Garbage => 0,
	})
	.sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUTP2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

	#[test]
	fn test1() {
		assert_eq!(part1(&parse(INPUT)), 161);
	}

    #[test]
	fn test2() {
		assert_eq!(part2(&parse(INPUTP2)), 48);
	}
    
  
}
