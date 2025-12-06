use fixedbitset::FixedBitSet;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

advent_of_code::solution!(4);

struct Grid {
    inner: FixedBitSet,
    width: usize,
    height: usize,
}

#[derive(EnumIter, Debug, Copy, Clone)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Grid {
    fn from_input(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let mut inner = FixedBitSet::with_capacity(width * height);

        for (y, line) in input.lines().enumerate() {
            for (x, bit) in line.chars().enumerate() {
                let index = y * width + x;

                match bit {
                    '.' => continue,
                    '@' => inner.set(index, true),
                    _ => panic!("Unknown value '{}'", bit),
                }
            }
        }

        Self {
            inner,
            width,
            height,
        }
    }

    fn move_towards(&self, index: usize, direction: Direction) -> Option<usize> {
        match direction {
            Direction::Up => {
                if index < self.width {
                    return None;
                }

                Some(index - self.width)
            }
            Direction::Down => {
                if index >= (self.height - 1) * self.width {
                    return None;
                }

                Some(index + self.width)
            }
            Direction::Left => {
                if index.rem_euclid(self.width) == 0 {
                    return None;
                }

                Some(index - 1)
            }
            Direction::Right => {
                if index.rem_euclid(self.width) == self.width - 1 {
                    return None;
                }

                Some(index + 1)
            }
            Direction::UpRight => {
                self.move_towards(self.move_towards(index, Direction::Up)?, Direction::Right)
            }
            Direction::DownRight => {
                self.move_towards(self.move_towards(index, Direction::Down)?, Direction::Right)
            }
            Direction::DownLeft => {
                self.move_towards(self.move_towards(index, Direction::Down)?, Direction::Left)
            }
            Direction::UpLeft => {
                self.move_towards(self.move_towards(index, Direction::Up)?, Direction::Left)
            }
        }
    }

    fn reachable(&self) -> Vec<usize> {
        self.inner
            .ones()
            .filter(|paper| {
                Direction::iter()
                    .filter_map(|direction| self.move_towards(*paper, direction))
                    .filter(|neighbour| self.inner.contains(*neighbour))
                    .count()
                    < 4
            })
            .collect()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(Grid::from_input(input).reachable().len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = Grid::from_input(input);

    let mut reachable = grid.reachable();
    let mut removed = 0;

    while !reachable.is_empty() {
        removed += reachable.len();

        for reachable in &reachable {
            grid.inner.remove(*reachable);
        }

        reachable = grid.reachable();
    }
    Some(removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
