use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Report {
    levels: Vec<i64>,
}

impl Report {
    fn sliding_window_check(levels: &[i64]) -> bool {
        let levels = &levels;
        let inc = levels[0] < levels[1];

        for i in 0..levels.len() - 1 {
            let [a, b] = &levels[i..(i + 2)] else {
                unreachable!()
            };

            let inc_dec_correct = if inc { a < b } else { a > b };

            let diff = (a - b).abs();
            let change_is_in_bounds = (1..=3).contains(&diff);

            if !inc_dec_correct || !change_is_in_bounds {
                return false;
            }
        }

        true
    }

    fn is_safe(&self) -> bool {
        Report::sliding_window_check(&self.levels)
    }

    fn is_safe_after_dampening(&self) -> bool {
        let len = self.levels.len();
        let levels = &self.levels;
        self.levels.iter().enumerate().any(|(i, _)| {
            let new_level = [&levels[0..i], &levels[(i + 1).min(len)..len]].concat();
            Report::sliding_window_check(&new_level)
        })
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
    reports.iter().filter(|report| report.is_safe()).count()
}

#[aoc(day2, part2)]
fn part2(reports: &[Report]) -> usize {
    reports
        .iter()
        .filter(|report| report.is_safe_after_dampening())
        .count()
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
