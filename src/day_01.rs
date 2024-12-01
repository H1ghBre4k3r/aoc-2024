use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Lists(Vec<i64>, Vec<i64>);

#[aoc_generator(day1)]
fn generator_day1(input: &str) -> Lists {
    let mut left = vec![];
    let mut right = vec![];

    input.split("\n").for_each(|line| {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        left.push(
            parts
                .first()
                .expect("Something went wrong")
                .parse()
                .expect("Something went wrong"),
        );
        right.push(
            parts
                .get(1)
                .expect("Something went wrong")
                .parse()
                .expect("Something went wrong"),
        );
    });

    Lists(left, right)
}

#[aoc(day1, part1)]
fn part1(input: &Lists) -> i64 {
    let Lists(mut left, mut right) = input.clone();

    assert_eq!(left.len(), right.len());

    left.sort();
    right.sort();

    let mut sum = 0;

    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs();
    }

    sum
}

#[aoc(day1, part2)]
fn part2(input: &Lists) -> i64 {
    let Lists(left, right) = input;

    let mut map = HashMap::<i64, i64>::new();

    for i in right {
        let current = map.get(i).cloned().unwrap_or_default();
        map.insert(*i, current + 1);
    }

    let mut sum = 0;

    for num in left {
        let occurences = map.get(num).cloned().unwrap_or_default();
        sum += *num * occurences;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::day_01::{part2, Lists};

    use super::{generator_day1, part1};

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_generator_part1() {
        let output = generator_day1(INPUT);

        assert_eq!(
            output,
            Lists(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])
        );
    }

    #[test]
    fn test_part1() {
        let gen = generator_day1(INPUT);
        let output = part1(&gen);

        assert_eq!(output, 11);
    }

    #[test]
    fn test_part2() {
        let gen = generator_day1(INPUT);
        let output = part2(&gen);

        assert_eq!(output, 31);
    }
}
