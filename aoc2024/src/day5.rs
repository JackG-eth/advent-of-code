use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse(input: &str) -> (HashMap<u64, HashSet<u64>>, Vec<Vec<u64>>) {
    let (section1, section2) = input.split_once("\n\n").unwrap();
    let mut ordering_rules: HashMap<u64, HashSet<u64>> = HashMap::new();

    section1.lines().for_each(|line| {
        let mut parts = line.split('|')
            .map(|num| num.trim().parse::<u64>().unwrap());
        if let (Some(p1), Some(p2)) = (parts.next(), parts.next()) {
            ordering_rules.entry(p1).or_insert_with(HashSet::new).insert(p2);
        }
    });

    let pages: Vec<Vec<u64>> = section2.lines().map(|line| line.split(',').map(|num| num.trim().parse::<u64>().unwrap()).collect::<Vec<u64>>()).collect();

    (ordering_rules, pages)
}

#[aoc(day5, part1)]
fn part1(input: &(HashMap<u64, HashSet<u64>>, Vec<Vec<u64>>)) -> u64 {
    let valid_pages = input.1.iter().filter_map(|update: &Vec<u64>| {
        let mut visited: HashSet<u64> = HashSet::new();
        let valid = update.iter().all({
            move |page| {
                if let Some(rules) = input.0.get(page){
                    visited.insert(*page);
                    rules.is_disjoint(&visited)
                } else {
                    visited.insert(*page);
                    true
                }
            }
        });
        if valid {
            Some(update)
        } else {
            None
        }
    }).collect::<Vec<_>>();

    valid_pages.iter().map(|update|update[update.len()/2]).sum()
}

#[aoc(day5, part2)]
fn part2(input: &(HashMap<u64, HashSet<u64>>, Vec<Vec<u64>>)) -> u64 {
    let invalid_pages = input.1.iter().filter_map(|update: &Vec<u64>| {
        let mut visited: HashSet<u64> = HashSet::new();
        let valid = update.iter().all({
            move |page| {
                if let Some(rules) = input.0.get(page){
                    visited.insert(*page);
                    rules.is_disjoint(&visited)
                } else {
                    visited.insert(*page);
                    true
                }
            }
        });
        if !valid {
            let mut fixed_update = update.clone();
            fixed_update.sort_by(|a, b| {
                // If a is in b's must-come-after set, a should come after b
                if let Some(b_rules) = input.0.get(b) {
                    if b_rules.contains(a) {
                        return std::cmp::Ordering::Greater;
                    }
                }
                // If b is in a's must-come-after set, b should come after a
                if let Some(a_rules) = input.0.get(a) {
                    if a_rules.contains(b) {
                        return std::cmp::Ordering::Less;
                    }
                }
                std::cmp::Ordering::Equal
            });
            Some(fixed_update)
        } else {
            None
        }
    }).collect::<Vec<_>>();

    invalid_pages.iter().map(|update|update[update.len()/2]).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

	const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

	#[test]
	fn test1() {
		assert_eq!(part1(&parse(INPUT)), 143);
	}
   
	#[test]
	fn test2() {
		assert_eq!(part2(&parse(INPUT)), 123);
	}
}
