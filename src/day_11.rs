use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Stone(u64);

impl Stone {
    fn blink(&self) -> Vec<Stone> {
        match self.0 {
            0 => vec![Stone(1)],
            x if x.ilog10() % 2 == 1 => {
                let len = x.ilog10() + 1;
                let mask = 10u64.pow(len / 2);

                let left = x / mask;
                let right = x % mask;

                vec![Stone(left), Stone(right)]
            }
            x => vec![Stone(x * 2024)],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Input {
    stones: Vec<Stone>,
}

#[aoc_generator(day11)]
fn generator(input: &str) -> Input {
    let stones = input
        .split_whitespace()
        .map(|s| Stone(s.parse().unwrap()))
        .collect();

    Input { stones }
}

#[aoc(day11, part1)]
fn part1(Input { stones }: &Input) -> u64 {
    let mut stone_map = HashMap::<Stone, u64>::new();
    for stone in stones {
        stone_map.insert(*stone, 1);
    }

    for _ in 0..25 {
        let mut new_stones = HashMap::new();
        for (stone, amount) in &stone_map {
            let res = stone.blink();

            for new_stone in &res {
                let current = new_stones.get(new_stone).cloned().unwrap_or_default();
                new_stones.insert(*new_stone, current + *amount);
            }
        }
        stone_map = new_stones;
    }

    let mut sum = 0;

    for amount in stone_map.values() {
        sum += *amount;
    }

    sum
}

#[aoc(day11, part2)]
fn part2(Input { stones }: &Input) -> u64 {
    let mut stone_map = HashMap::<Stone, u64>::new();
    for stone in stones {
        stone_map.insert(*stone, 1);
    }

    for _ in 0..75 {
        let mut new_stones = HashMap::new();
        for (stone, amount) in &stone_map {
            let res = stone.blink();

            for new_stone in &res {
                let current = new_stones.get(new_stone).cloned().unwrap_or_default();
                new_stones.insert(*new_stone, current + *amount);
            }
        }
        stone_map = new_stones;
    }

    let mut sum = 0;

    for amount in stone_map.values() {
        sum += *amount;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::day_11::{part1, Input, Stone};

    use super::generator;

    const INPUT: &str = "0 1 10 99 999";

    #[test]
    fn test_generator() {
        let gen = generator(INPUT);

        assert_eq!(
            gen,
            Input {
                stones: vec![Stone(0), Stone(1), Stone(10), Stone(99), Stone(999)]
            }
        );
    }

    #[test]
    fn test_blink() {
        assert_eq!(Stone(0).blink(), vec![Stone(1)]);
        assert_eq!(Stone(1).blink(), vec![Stone(2024)]);
        assert_eq!(Stone(2024).blink(), vec![Stone(20), Stone(24)]);
        assert_eq!(Stone(123456).blink(), vec![Stone(123), Stone(456)]);
    }

    #[test]
    fn test_part1() {
        let gen = generator("125 17");

        let res = part1(&gen);

        assert_eq!(res, 55312);
    }
}
