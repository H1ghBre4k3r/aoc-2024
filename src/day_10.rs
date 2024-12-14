use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::Coord;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Trails {
    map: Vec<Vec<u32>>,
}

impl Trails {
    fn at(&self, Coord(x, y): Coord) -> Option<u32> {
        self.map
            .get(y as usize)
            .and_then(|line| line.get(x as usize))
            .copied()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    map: Trails,
    starts: Vec<Coord>,
}

#[aoc_generator(day10)]
fn generator(input: &str) -> Input {
    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut starts = vec![];

    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 0 {
                starts.push(Coord(x as i64, y as i64));
            }
        }
    }

    Input {
        map: Trails { map },
        starts,
    }
}

fn backtrack(current: Coord, map: &Trails, ends: &mut HashSet<Coord>) -> usize {
    let Some(num) = map.at(current) else {
        unreachable!("This should not happen...");
    };

    let mut sum = 0;

    if num == 9 {
        ends.insert(current);
        return 1;
    }

    if let Some(up) = map.at(current.up()) {
        if up == num + 1 {
            sum += backtrack(current.up(), map, ends);
        }
    };

    if let Some(right) = map.at(current.right()) {
        if right == num + 1 {
            sum += backtrack(current.right(), map, ends);
        }
    };

    if let Some(down) = map.at(current.down()) {
        if down == num + 1 {
            sum += backtrack(current.down(), map, ends);
        }
    };

    if let Some(left) = map.at(current.left()) {
        if left == num + 1 {
            sum += backtrack(current.left(), map, ends);
        }
    };

    sum
}

#[aoc(day10, part1)]
fn part1(Input { map, starts }: &Input) -> usize {
    let mut sum = 0;

    for start in starts {
        let mut ends = HashSet::new();

        backtrack(*start, map, &mut ends);

        sum += ends.len();
    }

    sum
}

#[aoc(day10, part2)]
fn part2(Input { map, starts }: &Input) -> usize {
    let mut sum = 0;

    for start in starts {
        let mut ends = HashSet::new();

        sum += backtrack(*start, map, &mut ends);
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::{day_10::part2, utils::Coord};

    use super::{generator, part1};

    const SIMPLE_INPUT: &str = "9990999
9991999
1112111
6543456
7111117
8111118
9111119";

    #[test]
    fn test_generator() {
        let gen = generator(SIMPLE_INPUT);

        assert_eq!(gen.starts, vec![Coord(3, 0)]);
        assert_eq!(gen.starts.len(), 1);
    }

    #[test]
    fn test_simple_input_part1() {
        let gen = generator(SIMPLE_INPUT);

        let res = part1(&gen);

        assert_eq!(res, 2);
    }

    const COMPLEX_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_complext_input_part1() {
        let gen = generator(COMPLEX_INPUT);

        let res = part1(&gen);

        assert_eq!(res, 36);
    }

    #[test]
    fn test_complext_input_part2() {
        let gen = generator(COMPLEX_INPUT);

        let res = part2(&gen);

        assert_eq!(res, 81);
    }
}
