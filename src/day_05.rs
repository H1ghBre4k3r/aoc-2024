use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Input {
    rules: Vec<(u64, u64)>,
    updates: Vec<Vec<u64>>,
}

#[aoc_generator(day5)]
fn generator(input: &str) -> Input {
    let sections = input.split("\n\n").collect::<Vec<_>>();
    let rules = sections[0];
    let updates = sections[1];

    let rules = rules
        .lines()
        .map(|lines| {
            let parts = lines.split("|").collect::<Vec<_>>();
            (
                parts[0].parse::<u64>().unwrap(),
                parts[1].parse::<u64>().unwrap(),
            )
        })
        .collect();

    let updates = updates
        .lines()
        .map(|line| {
            line.split(",")
                .map(|update| update.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    Input { rules, updates }
}

#[aoc(day5, part1)]
fn part1(Input { rules, updates }: &Input) -> u64 {
    let mut blockers = HashMap::<u64, Vec<u64>>::new();

    for (left, right) in rules {
        if !blockers.contains_key(right) {
            blockers.insert(*right, vec![]);
        }
        let numbers = blockers.get_mut(right).unwrap();
        numbers.push(*left);
    }

    let mut sum = 0;

    for update in updates {
        let mut occured = HashSet::<u64>::new();
        let mut is_blocked = HashSet::<u64>::new();

        let mut is_valid = true;

        for num in update {
            if is_blocked.contains(num) {
                is_valid = false;
                break;
            }

            occured.insert(*num);

            if let Some(blocking) = blockers.get(num) {
                for block in blocking {
                    if !occured.contains(block) {
                        is_blocked.insert(*block);
                    }
                }
            }
        }

        if is_valid {
            sum += update[update.len() / 2];
        }
    }

    sum
}

#[aoc(day5, part2)]
fn part2(Input { rules, updates }: &Input) -> u64 {
    let mut blockers = HashMap::<u64, Vec<u64>>::new();

    for (left, right) in rules {
        if !blockers.contains_key(right) {
            blockers.insert(*right, vec![]);
        }
        let numbers = blockers.get_mut(right).unwrap();
        numbers.push(*left);
    }

    let mut sum = 0;

    for update in updates {
        let mut occured = HashSet::<u64>::new();
        let mut is_blocked = HashSet::<u64>::new();

        let mut is_valid = true;

        for num in update {
            if is_blocked.contains(num) {
                is_valid = false;
                break;
            }

            occured.insert(*num);

            if let Some(blocking) = blockers.get(num) {
                for block in blocking {
                    if !occured.contains(block) {
                        is_blocked.insert(*block);
                    }
                }
            }
        }

        if !is_valid {
            let mut update = update.clone();
            update.sort_by(|left, right| {
                let Some(blockers) = blockers.get(right) else {
                    return Ordering::Equal;
                };

                if blockers.contains(left) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            sum += update[update.len() / 2];
        }
    }

    sum
}

#[cfg(test)]
mod tests {

    use crate::day_05::part2;

    use super::{generator, part1};

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
    fn test_generator() {
        let gen = generator(INPUT);

        assert_eq!(gen.rules[0], (47, 53));
        assert_eq!(gen.updates[0], vec![75, 47, 61, 53, 29]);
    }

    #[test]
    fn test_part1() {
        let gen = generator(INPUT);
        let res = part1(&gen);

        assert_eq!(res, 143);
    }

    #[test]
    fn test_part2() {
        let gen = generator(INPUT);
        let res = part2(&gen);

        assert_eq!(res, 123);
    }
}
