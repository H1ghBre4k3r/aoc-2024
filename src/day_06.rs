use std::{collections::HashSet, ops::Add};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord(i64, i64);

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map(Vec<Vec<char>>);

impl Map {
    fn contains(&self, Coord(x, y): Coord) -> bool {
        let map = &self.0;
        y < (map.len() as i64) && y >= 0 && x < (map[0].len() as i64) && x >= 0
    }

    fn is_wall(&self, coord: Coord) -> bool {
        let Some(c) = self.get(coord) else {
            return false;
        };

        c == '#'
    }

    fn get(&self, Coord(x, y): Coord) -> Option<char> {
        self.0
            .get(y as usize)
            .and_then(|map| map.get(x as usize))
            .cloned()
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
    guard: Guard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Guard {
    position: Coord,
    direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
        guard: guard.unwrap(),
    }
}

#[aoc(day6, part1)]
fn part1(Input { map, guard, .. }: &Input) -> usize {
    let Guard {
        position,
        direction,
    } = guard;

    let mut map = map.clone();

    let mut position = *position;
    let mut direction = *direction;

    while map.contains(position) {
        map.mark(position);

        let new_position = position + direction.offset();

        if map.is_wall(new_position) {
            direction = direction.turn();
            position = position + direction.offset();
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

#[aoc(day6, part2)]
fn part2(Input { map, guard, .. }: &Input) -> usize {
    let Guard {
        position,
        direction,
    } = guard;
    let start = *position;

    let mut position = *position;
    let mut direction = *direction;

    let mut sum = 0;

    let mut already_tested = HashSet::new();

    while map.contains(position) {
        let new_position = position + direction.offset();
        if map.is_wall(new_position) {
            direction = direction.turn();
            continue;
        }

        // if there is no wall, we just place to obstacle right in front of us
        let obstacle = new_position;

        // only count each successful location once and do not test start
        if !already_tested.contains(&obstacle) && obstacle != start && map.contains(obstacle) {
            // track visited positions and directions
            let mut visited = HashSet::new();

            let mut sim_position = position;

            // since we know that there is an obstacle, we just turn right
            let mut sim_direction = direction.turn();

            while map.contains(sim_position) {
                let current_visiting = (sim_position, sim_direction);
                // check, if we've already been here
                if visited.contains(&current_visiting) {
                    already_tested.insert(obstacle);
                    sum += 1;
                    break;
                }
                visited.insert(current_visiting);

                // check for wall or our obstacle
                let new_position = sim_position + sim_direction.offset();
                if map.is_wall(new_position) || new_position == obstacle {
                    sim_direction = sim_direction.turn();
                    continue;
                }

                sim_position = new_position;
            }
        }

        position = new_position;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::day_06::{Coord, Direction, Guard};

    use super::{generator, part1, part2};

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

    #[test]
    fn test_part2() {
        let gen = generator(INPUT);

        let res = part2(&gen);

        assert_eq!(res, 6);
    }

    /*
     * This can create three different loops:
     *
     * ...#0....
     * ...++--+#
     * .#.||..|.
     * ...||.#|.
     * ...||..|.
     * ..#+---+.
     * ....|..#.
     * #...|....
     * ....^##..
     *
     * ...#.....
     * ........#
     * .#..0....
     * .+--++#..
     * .|..||...
     * .|#.||...
     * .|..||.#.
     * #+---+...
     * ....^##..
     *
     * ...#.....
     * ........#
     * .#.......
     * .+---+#..
     * .|...|...
     * .|#.0|...
     * .|..+-+#.
     * #+---++..
     * ....^##..
     */
    const OTHER: &str = "...#.....
........#
.#.......
......#..
.........
..#......
.......#.
#........
....^##..";

    #[test]
    fn test_other() {
        let gen = generator(OTHER);

        let res = part1(&gen);
        assert_eq!(res, 9);

        let res = part2(&gen);
        assert_eq!(res, 3);
    }
}
