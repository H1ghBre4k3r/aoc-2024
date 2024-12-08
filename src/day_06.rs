use std::ops::{Add, AddAssign};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord(i64, i64);

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map(Vec<Vec<char>>);

impl Map {
    fn contains(&self, Coord(x, y): Coord) -> bool {
        let map = &self.0;
        y < (map.len() as i64) && y >= 0 && x < (map[0].len() as i64) && x >= 0
    }

    fn is_wall(&self, Coord(x, y): Coord) -> bool {
        let map = &self.0;
        let Some(c) = map.get(y as usize).and_then(|map| map.get(x as usize)) else {
            return false;
        };

        *c == '#'
    }

    fn mark(&mut self, Coord(x, y): Coord) {
        let Some(c) = self.0.get_mut(y as usize) else {
            return;
        };

        c[x as usize] = 'X';
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    map: Map,
    dim: (i64, i64),
    guard: Guard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Guard {
    position: Coord,
    direction: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn turn(&self) -> Direction {
        use Direction::*;

        match self {
            Up => Right,
            Down => Left,
            Right => Down,
            Left => Up,
        }
    }

    fn offset(&self) -> Coord {
        use Direction::*;

        match self {
            Up => Coord(0, -1),
            Down => Coord(0, 1),
            Right => Coord(1, 0),
            Left => Coord(-1, 0),
        }
    }
}

#[aoc_generator(day6)]
fn generator(input: &str) -> Input {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let width = map[0].len();
    let height = map.len();

    let mut guard = None;
    for x in 0..width {
        for y in 0..height {
            let position = Coord(x as i64, y as i64);
            let c = map[y][x];

            match c {
                '^' => {
                    guard = Some(Guard {
                        position,
                        direction: Direction::Up,
                    });
                }
                '>' => {
                    guard = Some(Guard {
                        position,
                        direction: Direction::Right,
                    })
                }
                'v' => {
                    guard = Some(Guard {
                        position,
                        direction: Direction::Down,
                    })
                }
                '<' => {
                    guard = Some(Guard {
                        position,
                        direction: Direction::Left,
                    })
                }
                _ => {}
            }
        }
    }

    Input {
        map: Map(map),
        dim: (width as i64, height as i64),
        guard: guard.unwrap(),
    }
}

#[aoc(day6, part1)]
fn part1(Input { map, dim, guard }: &Input) -> usize {
    let Guard {
        position,
        direction,
    } = guard;

    let mut map = map.clone();

    let mut position = *position;
    let mut direction = direction.clone();

    while map.contains(position) {
        map.mark(position);

        let new_position = position + direction.offset();

        if map.is_wall(new_position) {
            direction = direction.turn();
            position += direction.offset();
        } else {
            position = new_position;
        }
    }

    let mut sum = 0;

    for row in &map.0 {
        for c in row {
            if *c == 'X' {
                sum += 1;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::day_06::{Coord, Direction, Guard};

    use super::{generator, part1};

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_generator() {
        let gen = generator(INPUT);

        assert_eq!(gen.dim, (10, 10));
        assert_eq!(
            gen.guard,
            Guard {
                position: Coord(4, 6),
                direction: Direction::Up
            }
        )
    }

    #[test]
    fn test_part1() {
        let gen = generator(INPUT);

        let res = part1(&gen);

        assert_eq!(res, 41);
    }
}
