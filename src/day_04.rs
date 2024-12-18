use std::ops::{Add, Mul};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    lines: Vec<Vec<char>>,
    dim: (i64, i64),
}

impl Input {
    fn at(&self, offset: Offset) -> Option<u8> {
        let Offset(x, y) = offset;

        self.lines
            .get(y as usize)
            .and_then(|line| line.get(x as usize).map(|c| *c as u8))
    }
}

#[aoc_generator(day04)]
fn generator(input: &str) -> Input {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let dim = (lines[0].len() as i64, lines.len() as i64);
    Input { lines, dim }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Offset(i64, i64);

impl Mul<i64> for Offset {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        let Offset(x, y) = self;
        Offset(x * rhs, y * rhs)
    }
}

impl Add for Offset {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Offset(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[aoc(day04, part1)]
fn part1(input: &Input) -> usize {
    let (width, height) = input.dim;

    let mut xmas = 0;

    let offsets = vec![
        Offset(-1, 0),
        Offset(1, 0),
        Offset(0, -1),
        Offset(0, 1),
        Offset(-1, -1),
        Offset(1, -1),
        Offset(-1, 1),
        Offset(1, 1),
    ];

    for x in 0..width {
        for y in 0..height {
            let index = Offset(x, y);
            let Some(c) = input.at(index) else {
                continue;
            };

            if c != b'X' {
                continue;
            }

            for offset in &offsets {
                let mut buffer = vec![];
                for i in 1..=3 {
                    if let Some(c) = input.at(index + *offset * i) {
                        buffer.push(c);
                    }
                }

                let text = String::from_utf8(buffer).unwrap();

                if text == "MAS" {
                    xmas += 1;
                }
            }
        }
    }

    xmas
}

#[aoc(day04, part2)]
fn part2(input: &Input) -> usize {
    let (width, height) = input.dim;

    let mut xmas = 0;

    for x in 0..width {
        for y in 0..height {
            let index = Offset(x, y);
            let Some(c) = input.at(index) else {
                continue;
            };

            if c != b'A' {
                continue;
            }

            let top_left = input.at(index + Offset(-1, -1));
            let bottom_right = input.at(index + Offset(1, 1));
            let first_diagonal = matches!(
                (top_left, bottom_right),
                (Some(b'M'), Some(b'S')) | (Some(b'S'), Some(b'M'))
            );

            let bottom_left = input.at(index + Offset(-1, 1));
            let top_right = input.at(index + Offset(1, -1));
            let second_diagonal = matches!(
                (bottom_left, top_right),
                (Some(b'M'), Some(b'S')) | (Some(b'S'), Some(b'M'))
            );

            if first_diagonal && second_diagonal {
                xmas += 1;
            }
        }
    }

    xmas
}

#[cfg(test)]
mod tests {
    use crate::day_04::part2;

    use super::{generator, part1};

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
    fn test_generator_part1() {
        let gen = generator(INPUT);

        assert_eq!(gen.dim, (10, 10));
    }

    #[test]
    fn test_part1() {
        let gen = generator(INPUT);

        let output = part1(&gen);

        assert_eq!(output, 18);
    }

    #[test]
    fn test_part2() {
        let gen = generator(INPUT);

        let output = part2(&gen);

        assert_eq!(output, 9);
    }
}
