use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub enum Direction {
    NORTH,
    SOUTH,
    WEST,
    EAST,
}

impl Default for Direction {
    fn default() -> Self {
        Self::NORTH
    }
}

impl Direction {
    fn rotate(&mut self) {
        *self = match  self {
            Direction::NORTH => Direction::EAST,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
            Direction::EAST => Direction::SOUTH,
        };
    }
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<Vec<char>>{
    input.lines().map(|line| {
            line.chars().collect()
        }
    ).collect()
}

#[aoc(day6, part1)]
fn part1(input: &[Vec<char>]) -> u64 {
    let (mut x,mut y) = (0,0);
    let mut count = 0;
    let mut visited: HashSet<(usize,usize)> = HashSet::new();

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == '^' {
                (x,y) = (i,j);
            }
        }
    }

    let mut direction = Direction::default();

    loop {
        // Mark current position as visited
        if !visited.contains(&(x, y)) {
            visited.insert((x, y));
            count += 1;
        }
        // Try to move in current direction
        match direction {
            Direction::NORTH => {
                if x == 0 { break; }
                if input[x-1][y] == '#' {
                    direction.rotate();
                    (x, y);
                }else {
                    (x -= 1, y);
                }
            },
            Direction::SOUTH => {
                if x >= input.len() - 1 { break; }
                if input[x+1][y] == '#' {
                    direction.rotate();
                    (x, y);
                }else {
                    (x += 1, y);
                }
            },
            Direction::WEST => {
                if y == 0 { break; }
                if input[x][y-1] == '#' {
                    direction.rotate();
                    (x, y);
                }else {
                    (x, y-=1);
                }
            },
            Direction::EAST => {
                if y >= input[x].len() - 1 { break; }
                if y == 0 { break; }
                if input[x][y+1] == '#' {
                    direction.rotate();
                    (x, y);
                }else {
                    (x, y+=1);
                }
            },
        };
    }

    count
}

#[aoc(day6, part2)]
fn part2(input: &[Vec<char>]) -> u64 {
  1
}


#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

	#[test]
	fn test1() {
		assert_eq!(part1(&parse(INPUT)), 41);
	}

	#[test]
	fn test2() {
		assert_eq!(part2(&parse(INPUT)), 6);
	}
}