use aoc_runner_derive::{aoc, aoc_generator};

pub struct Equation {
    pub total: u64,
    pub inputs: Vec<u64>
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .filter_map(|line| {
            line.split_once(':').map(|(total, nums_line)| {
                let nums: Vec<u64> = nums_line
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();
                Equation {
                    total: total.parse().unwrap(),
                    inputs: nums,
                }
            })
        })
        .collect()
}

///1 2 3
/// 1 + 2

#[aoc(day7, part1)]
fn part1(input: &[Equation]) -> u64 {
    let mut total = 0;
    
    for eq in input.iter() {
        let target = eq.total;
        let nums = &eq.inputs;
        let n = nums.len();
        
        // Iterate over all combinations of operations 0..
        for operator_combination in 0..(1 << (n - 1)) { 
            let mut result = nums[0];
            
            for i in 1..n {
                let operator = if (operator_combination & (1 << (i - 1))) != 0 { 1 } else { 0 };
                println!("operator {}", operator);
                match operator {
                    0 => result *= nums[i], // Try multiplication
                    1 => result += nums[i], // Try addition
                    _ => unreachable!(),
                }
            }
            
            if result == target {
                total += target;
                break;
            }
        }
    }
    
    total
}

#[aoc(day7, part2)]
fn part2(nput: &[Equation]) -> u64 {
0
}

#[cfg(test)]
mod tests {
	use super::*;

    // two operators 
    // 9 7 18 13 3^2 (9 combos)

	const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

	#[test]
	fn test1() {
		assert_eq!(part1(&parse(INPUT)), 3749);
	}

	#[test]
	fn test2() {
		assert_eq!(part2(&parse(INPUT)), 6);
	}
}