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
        if !visited.contains(&(x, y)) {
            visited.insert((x, y));
            count += 1;
        }

        match direction {
            Direction::NORTH => {
                if x == 0 { break; }
                if input[x-1][y] == '#' {
                    direction.rotate();
                } else {
                    x -= 1;
                }
            },
            Direction::SOUTH => {
                if x >= input.len() - 1 { break; }
                if input[x+1][y] == '#' {
                    direction.rotate();
                } else {
                    x += 1;
                }
            },
            Direction::WEST => {
                if y == 0 { break; }
                if input[x][y-1] == '#' {
                    direction.rotate();
                } else {
                    y -= 1;
                }
            },
            Direction::EAST => {
                if y >= input[x].len() - 1 { break; }
                if input[x][y+1] == '#' {
                    direction.rotate();
                } else {
                    y += 1;
                }
            },
        }
    }
    count
}

#[aoc(day6, part2)]
fn part2(input: &[Vec<char>]) -> u64 {
    let (start_x, start_y) = find_start(input);
    let mut count = 0;
    
    // Try each empty position
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == '.' && attempt(input, (i, j), (start_x, start_y)) {
                count += 1;
            }
        }
    }
    count
}

fn attempt(grid: &[Vec<char>], wall_pos: (usize, usize), start: (usize, usize)) -> bool {
    if wall_pos == start {
        return false;
    }

    let (mut x, mut y) = (start.0 as i32, start.1 as i32);
    let mut dx: i32 = 0;
    let mut dy: i32 = -1;  // Start moving north
    let mut visited = HashSet::new();

    while x >= 0 && y >= 0 && x < grid.len() as i32 && y < grid[0].len() as i32 {
        // Check if we've seen this state before (position + direction)
        if visited.contains(&(x, y, dx, dy)) {
            return true;
        }
        visited.insert((x, y, dx, dy));

        let next_x = x + dx;
        let next_y = y + dy;

        // Check if we hit a wall (including our new wall) or boundary
        if next_x < 0 || next_y < 0 
           || next_x >= grid.len() as i32 
           || next_y >= grid[0].len() as i32
           || grid[next_x as usize][next_y as usize] == '#'
           || (next_x as usize, next_y as usize) == wall_pos {
            // Rotate clockwise
            let temp_dx = dx;
            dx = -dy;
            dy = temp_dx;
        } else {
            x = next_x;
            y = next_y;
        }
    }
    false
}

fn find_start(grid: &[Vec<char>]) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '^' {
                return (i, j);
            }
        }
    }
    (0, 0)
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