use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Mem {
    E,
    S(usize, usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Input {
    mem: Vec<Mem>,
}

#[aoc_generator(day9)]
fn generator(input: &str) -> Input {
    let mem = input
        .chars()
        .enumerate()
        .flat_map(|(n, c)| {
            let num = c.to_digit(10).unwrap() as usize;

            if n % 2 == 1 {
                vec![Mem::E; num]
            } else {
                vec![Mem::S(n / 2, num); num]
            }
        })
        .collect();

    Input { mem }
}

#[aoc(day9, part1)]
fn part1(Input { mem }: &Input) -> usize {
    let mut mem = mem.clone();
    let mut right = mem.len() - 1;

    while let Mem::E = mem[right] {
        right -= 1;
    }

    let mut sum = 0;

    for i in 0..mem.len() {
        if i > right {
            break;
        }

        let current = mem[i];

        let Mem::E = current else {
            continue;
        };

        mem[i] = mem[right];
        mem[right] = current;

        while let Mem::E = mem[right] {
            right -= 1;
        }
    }

    for (i, m) in mem.iter().enumerate() {
        let Mem::S(n, _) = m else {
            break;
        };

        sum += i * *n;
    }

    sum
}

fn find_empty_space_with_size(mem: &[Mem], size: usize, max: usize) -> Option<usize> {
    for i in 0..max {
        let mut found = true;
        for j in i..i + size {
            if let Mem::S(_, _) = mem[j] {
                found = false;
                break;
            }
        }

        if found {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
fn print_mem(mem: &[Mem]) {
    for m in mem {
        match m {
            Mem::S(n, _) => print!("{n}"),
            Mem::E => print!("."),
        }
    }

    println!();
}

#[aoc(day9, part2)]
fn part2(Input { mem }: &Input) -> usize {
    let mut mem = mem.clone();

    let mut right = mem.len() - 1;

    while right > 0 {
        #[cfg(test)]
        print_mem(&mem);
        let current = mem[right];
        let Mem::S(_, size) = current else {
            right -= 1;
            continue;
        };

        if right <= size {
            break;
        }

        let Some(left) = find_empty_space_with_size(&mem, size, right - size) else {
            right -= size;
            continue;
        };

        for i in 0..size {
            mem.swap(right + i - (size - 1), left + i);
        }

        right -= size;
    }

    let mut sum = 0;

    for (i, m) in mem.iter().enumerate() {
        let Mem::S(n, _) = m else {
            continue;
        };

        sum += i * *n;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::day_09::{part2, Input, Mem};

    use super::{generator, part1};

    #[test]
    fn test_generator() {
        let gen = generator("12345");

        assert_eq!(
            gen,
            Input {
                mem: vec![
                    Mem::S(0, 1),
                    Mem::E,
                    Mem::E,
                    Mem::S(1, 3),
                    Mem::S(1, 3),
                    Mem::S(1, 3),
                    Mem::E,
                    Mem::E,
                    Mem::E,
                    Mem::E,
                    Mem::S(2, 5),
                    Mem::S(2, 5),
                    Mem::S(2, 5),
                    Mem::S(2, 5),
                    Mem::S(2, 5),
                ]
            }
        );
    }

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        let gen = generator(INPUT);

        let res = part1(&gen);

        assert_eq!(res, 1928);
    }

    #[test]
    fn test_part2() {
        let gen = generator(INPUT);

        let res = part2(&gen);

        assert_eq!(res, 2858);
    }
}
