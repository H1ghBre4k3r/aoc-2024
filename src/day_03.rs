use aoc_runner_derive::{aoc, aoc_generator};
use regex::{Captures, Regex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Mul(i64, i64);

impl Mul {
    fn calc(&self) -> i64 {
        self.0 * self.1
    }
}

#[aoc_generator(day3, part1)]
fn generator_part1(input: &str) -> Vec<Mul> {
    let re = Regex::new(r"(?m)mul\(([0-9]+),*([0-9]+)\)").unwrap();

    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [left, right])| Mul(left.parse().unwrap(), right.parse().unwrap()))
        .collect()
}

#[aoc(day3, part1)]
fn part1(muls: &[Mul]) -> i64 {
    muls.iter().map(Mul::calc).sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Mul(Mul),
    Do,
    Dont,
}

impl<'h> From<Captures<'h>> for Instruction {
    fn from(cap: Captures<'h>) -> Self {
        match &cap[0] {
            "do()" => Instruction::Do,
            "don't()" => Instruction::Dont,
            mul if mul.starts_with("mul(") => {
                Instruction::Mul(Mul(cap[2].parse().unwrap(), cap[3].parse().unwrap()))
            }
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day3, part2)]
fn generator_part2(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"(?m)(mul\(([0-9]+),*([0-9]+)\)|(don't\(\))|(do\(\)))").unwrap();
    re.captures_iter(input).map(Instruction::from).collect()
}

#[aoc(day3, part2)]
fn part2(instructions: &[Instruction]) -> i64 {
    let mut sum = 0;
    let mut cando = true;
    instructions
        .iter()
        .for_each(|instruction| match instruction {
            Instruction::Mul(mul) if cando => sum += mul.calc(),
            Instruction::Do => cando = true,
            Instruction::Dont => cando = false,
            _ => {}
        });

    sum
}

#[cfg(test)]
mod tests {
    use crate::day_03::{generator_part2, Instruction, Mul};

    use super::{generator_part1, part1, part2};

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_generator_part1() {
        let gen = generator_part1(INPUT);

        let expected = vec![Mul(2, 4), Mul(5, 5), Mul(11, 8), Mul(8, 5)];

        assert_eq!(gen, expected);
    }

    #[test]
    fn test_part1() {
        let gen = generator_part1(INPUT);

        let result = part1(&gen);

        assert_eq!(result, 161);
    }

    const INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_generator_part2() {
        let gen = generator_part2(INPUT2);
        let expected = vec![
            Instruction::Mul(Mul(2, 4)),
            Instruction::Dont,
            Instruction::Mul(Mul(5, 5)),
            Instruction::Mul(Mul(11, 8)),
            Instruction::Do,
            Instruction::Mul(Mul(8, 5)),
        ];

        assert_eq!(gen, expected);
    }

    #[test]
    fn test_part2() {
        let gen = generator_part2(INPUT2);
        let result = part2(&gen);

        assert_eq!(result, 48);
    }
}
