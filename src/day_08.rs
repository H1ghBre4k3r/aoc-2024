use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::Coord;

type Antennas = HashMap<char, Vec<Coord>>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    antennas: Antennas,
    dim: (i64, i64),
}

#[aoc_generator(day8)]
fn generator(input: &str) -> Input {
    let mut antennas = Antennas::new();

    let mut height = 0;
    let mut width = 0;

    for (y, line) in input.lines().enumerate() {
        // very inefficient way to calculate width and height
        // do not perform this at home!
        height = y + 1;
        let mut inner_width = 0;
        for (x, c) in line.chars().enumerate() {
            inner_width = x + 1;
            if c == '.' {
                continue;
            }

            let antenna = Coord(x as i64, y as i64);

            let mut others = antennas.get(&c).cloned().unwrap_or_default();
            others.push(antenna);
            antennas.insert(c, others);
        }
        width = inner_width;
    }

    Input {
        antennas,
        dim: (width as i64, height as i64),
    }
}

fn get_single_antinode(a: Coord, b: Coord, (width, height): (i64, i64)) -> Option<Coord> {
    let antinode @ Coord(x, y) = a + (a - b);

    if x >= 0 && x < width && y >= 0 && y < height {
        Some(antinode)
    } else {
        None
    }
}

#[aoc(day8, part1)]
fn part1(Input { antennas, dim }: &Input) -> usize {
    let mut antinodes = HashSet::<Coord>::new();

    for antennas in antennas.values() {
        for a in antennas {
            for b in antennas {
                if a == b {
                    continue;
                }

                if let Some(antinode) = get_single_antinode(*a, *b, *dim) {
                    antinodes.insert(antinode);
                }

                if let Some(antinode) = get_single_antinode(*b, *a, *dim) {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    antinodes.len()
}

fn get_all_antinodes(a: Coord, b: Coord, dim: (i64, i64)) -> Vec<Coord> {
    let diff = a - b;

    let mut antinodes = vec![];

    let mut n = 1;
    while let Some(antinode) = get_single_antinode(a, b + (diff * n), dim) {
        antinodes.push(antinode);
        n += 1;
    }

    antinodes
}

#[aoc(day8, part2)]
fn part2(Input { antennas, dim }: &Input) -> usize {
    let mut antinodes = HashSet::<Coord>::new();

    for antennas in antennas.values() {
        for a in antennas {
            for b in antennas {
                if a == b {
                    continue;
                }

                for antinode in &get_all_antinodes(*a, *b, *dim) {
                    antinodes.insert(*antinode);
                }

                for antinode in &get_all_antinodes(*b, *a, *dim) {
                    antinodes.insert(*antinode);
                }
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use crate::day_08::part2;

    use super::{generator, part1, Antennas, Coord, Input};

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_generator() {
        let gen = generator(INPUT);

        let mut antennas = Antennas::new();
        let zero_labelled = vec![Coord(8, 1), Coord(5, 2), Coord(7, 3), Coord(4, 4)];
        antennas.insert('0', zero_labelled);

        let a_labelled = vec![Coord(6, 5), Coord(8, 8), Coord(9, 9)];
        antennas.insert('A', a_labelled);

        assert_eq!(
            gen,
            Input {
                antennas,
                dim: (12, 12)
            }
        );
    }

    #[test]
    fn test_part1() {
        let gen = generator(INPUT);

        let res = part1(&gen);

        assert_eq!(res, 14);
    }

    #[test]
    fn test_part2() {
        let gen = generator(INPUT);

        let res = part2(&gen);

        assert_eq!(res, 34);
    }
}
