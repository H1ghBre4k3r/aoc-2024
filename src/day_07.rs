use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    equations: Vec<Equation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Equation {
    result: u64,
    operands: Vec<u64>,
}

#[aoc_generator(day7)]
fn generator(input: &str) -> Input {
    let equations = input
        .lines()
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<_>>();
            let result = parts[0].parse::<u64>().unwrap();
            let operands = parts[1]
                .split_whitespace()
                .map(|op| op.parse::<u64>().unwrap())
                .collect();
            Equation { result, operands }
        })
        .collect();

    Input { equations }
}

fn backtrack(
    target: u64,
    current: Option<u64>,
    others: &[u64],
    funs: &Vec<&dyn Fn(u64, u64) -> u64>,
) -> u64 {
    if others.is_empty() {
        return current.expect("oopsie daisy");
    }

    let rest = if others.len() == 1 { &[] } else { &others[1..] };

    match current {
        Some(current) => {
            for fun in funs {
                let res = fun(current, others[0]);
                let next = backtrack(target, Some(res), rest, funs);

                if next == target {
                    return next;
                }
            }

            0
        }
        None => backtrack(target, Some(others[0]), rest, funs),
    }
}

fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn mul(a: u64, b: u64) -> u64 {
    a * b
}

#[aoc(day7, part1)]
fn part1(Input { equations }: &Input) -> u64 {
    let mut sum = 0;

    let funs: Vec<&dyn Fn(u64, u64) -> u64> = vec![&add, &mul];

    for Equation { result, operands } in equations {
        let result = *result;
        let new_res = backtrack(result, None, operands, &funs);

        if result == new_res {
            sum += result;
        }
    }

    sum
}

fn append(a: u64, b: u64) -> u64 {
    let new = format!("{a}{b}");

    new.parse().unwrap()
}

#[aoc(day7, part2)]
fn part2(Input { equations }: &Input) -> u64 {
    let mut sum = 0;

    let funs: Vec<&dyn Fn(u64, u64) -> u64> = vec![&add, &mul, &append];

    for Equation { result, operands } in equations {
        let result = *result;
        let new_res = backtrack(result, None, operands, &funs);

        if result == new_res {
            sum += result;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::day_07::{Equation, Input};

    use super::{generator, part1, part2};

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
    fn test_generator() {
        let gen = generator(INPUT);

        assert_eq!(
            gen,
            Input {
                equations: vec![
                    Equation {
                        result: 190,
                        operands: vec![10, 19]
                    },
                    Equation {
                        result: 3267,
                        operands: vec![81, 40, 27]
                    },
                    Equation {
                        result: 83,
                        operands: vec![17, 5]
                    },
                    Equation {
                        result: 156,
                        operands: vec![15, 6]
                    },
                    Equation {
                        result: 7290,
                        operands: vec![6, 8, 6, 15]
                    },
                    Equation {
                        result: 161011,
                        operands: vec![16, 10, 13]
                    },
                    Equation {
                        result: 192,
                        operands: vec![17, 8, 14]
                    },
                    Equation {
                        result: 21037,
                        operands: vec![9, 7, 18, 13]
                    },
                    Equation {
                        result: 292,
                        operands: vec![11, 6, 16, 20]
                    }
                ]
            }
        )
    }

    #[test]
    fn test_part1() {
        let gen = generator(INPUT);

        let res = part1(&gen);

        assert_eq!(res, 3749);
    }

    #[test]
    fn test_part2() {
        let gen = generator(INPUT);

        let res = part2(&gen);

        assert_eq!(res, 11387);
    }
}
