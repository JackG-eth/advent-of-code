use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Vec<char>> {
  input.lines().map(|l| l.chars().collect()).collect()
}



#[aoc(day4, part1)]
fn part1(m: &[Vec<char>]) -> u32 {
    let patterns = [
        // Horizontal patterns
        [(0,0), (0,1), (0,2), (0,3)],
        // Vertical patterns
        [(0,0), (1,0), (2,0), (3,0)],
        // Diagonal patterns
        [(0,0), (1,1), (2,2), (3,3)],
        [(0,3), (1,2), (2,1), (3,0)],
    ];

    let words = [
        ['X', 'M', 'A', 'S'],
        ['S', 'A', 'M', 'X'],
    ];

    let mut count = 0;
    let height = m.len();
    let width = m[0].len();

    for y in 0..height {
        for x in 0..width {
            for pattern in patterns.iter() {
              
                let valid = pattern.iter().all(|&(dy, dx)| {
                    let ny = y as i32 + dy as i32;
                    let nx = x as i32 + dx as i32;
                    ny >= 0 && ny < height as i32 && nx >= 0 && nx < width as i32
                });

                if !valid {
                    continue;
                }

                for word in words.iter() {
                    if pattern.iter().enumerate().all(|(i, &(dy, dx))| {
                        m[y + dy][x + dx] == word[i]
                    }) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}


#[aoc(day4, part2)]
fn part2(m: &[Vec<char>]) -> u32 {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

	const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

	#[test]
	fn test1() {
		assert_eq!(part1(&parse(INPUT)), 18);
	}
   
	#[test]
	fn test2() {
		assert_eq!(part2(&parse(INPUT)), 9);
	}
}
