use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Report {
    levels: Vec<i64>,
}

fn check_two_numbers(a: i64, b: i64, inc_global: &mut Option<bool>) -> bool {
    let inc = inc_global.get_or_insert(a < b);
    let inc_dec_correct = if *inc { a < b } else { a > b };

    let diff = (a - b).abs();
    let change_is_in_bounds = (1..=3).contains(&diff);

    inc_dec_correct && change_is_in_bounds
}

impl Report {
    fn is_safe(&self, mut can_skip: bool) -> bool {
        let levels = &self.levels;

        let len = levels.len();

        let mut i = 0;

        let mut inc_global = None;

        while i < len - 1 {
            let old_inc = inc_global;

            let a = levels[i];
            let b = levels[i + 1];
            if check_two_numbers(a, b, &mut inc_global) {
                // the current two numbers fit, just continue
                i += 1;
                continue;
            } else if i == len - 2 && can_skip {
                // we are at the end and did not skip yet
                return true;
            }
            // if we have an error, we need to reset it since we might jump
            inc_global = old_inc;

            if !can_skip || i == len - 2 {
                // we are at the and or can not skip anymore
                return false;
            }
            can_skip = false;

            let c = levels[i + 2];
            // if we are at the start, we can simply ignore the first level
            if i == 0 && check_two_numbers(b, c, &mut inc_global) {
                i += 1;
                continue;
            }
            inc_global = old_inc;

            if check_two_numbers(a, c, &mut inc_global) {
                // we can skip b and move directly to c
                i += 2;
                continue;
            }

            return false;
        }

        true
    }
}

#[aoc_generator(day2)]
fn generator_day2(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|line| Report {
            levels: line
                .split_whitespace()
                .map(|level| level.parse::<i64>().expect("Something went wrong"))
                .collect(),
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(reports: &[Report]) -> usize {
    reports
        .iter()
        .filter(|report| report.is_safe(false))
        .count()
}

#[aoc(day2, part2)]
fn part2(reports: &[Report]) -> usize {
    reports.iter().filter(|report| report.is_safe(true)).count()
}

#[cfg(test)]
mod tests {

    use super::{generator_day2, part1, part2, Report};

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_generator_day2() {
        let gen = generator_day2(INPUT);

        let expected = vec![
            Report {
                levels: vec![7, 6, 4, 2, 1],
            },
            Report {
                levels: vec![1, 2, 7, 8, 9],
            },
            Report {
                levels: vec![9, 7, 6, 2, 1],
            },
            Report {
                levels: vec![1, 3, 2, 4, 5],
            },
            Report {
                levels: vec![8, 6, 4, 4, 1],
            },
            Report {
                levels: vec![1, 3, 6, 7, 9],
            },
        ];

        assert_eq!(gen, expected)
    }

    #[test]
    fn test_part1() {
        let gen = generator_day2(INPUT);
        let result = part1(&gen);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2() {
        let gen = generator_day2(INPUT);
        let result = part2(&gen);

        assert_eq!(result, 4);
    }
}
